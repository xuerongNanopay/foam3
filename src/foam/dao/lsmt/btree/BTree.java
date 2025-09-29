/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.*;

import foam.dao.lsmt.utils.BulkIterator;

/**
 * Copy-on-Write Btree.
 *  - No in-place update. Caller should use CAS for thread-safe.
 *  - Max support child size is limited by: tree_height * FANOUT_SHIFT < 32.
 *  - key and value doesn't not seperate, they are wrapped in the Object.
 *  - The entire btree is structed in Object array.
 *    - The leaf array must be ODD in length.(if the size of leaf is even, it will append extra NULL element at the end.)
 *    - The internal array must be EVEN in length. The last element in the internal array will be zoom map.
 *        - layout: [key1, key2, ... keyN, child1, child2, .... childN+1, ZoomMap]
 *        - child is reference to sub tree(reference to Object Array).
 *        - algorithm will guarantee the child size is ODD, in order to make sure the array length of internal node is EVEN.
 */

public class BTree {

  //TODO: configure FANOUT_SHIFT.
  public static final int FANOUT_SHIFT = 5;
  private static final int FANOUT = 1 << FANOUT_SHIFT;
  public static final int MIN_TUPLES = FANOUT / 2 - 1;
  public static final int MAX_TUPLES = FANOUT - 1;

  private static final Object[] EMPTY_LEAF = new Object[1];

  public static Object[] empty() {
    return EMPTY_LEAF;
  }

  public static Object[] singleton(Object val) {
    return new Object[]{ val };
  }

  public static <T> Object[] build(BulkIterator<T> sortedBulk, int size) {
    if ( size == 0 ) return EMPTY_LEAF;
    if ( size <= MAX_TUPLES ) return buildLeaf(sortedBulk, size);

    // return buildRoot();
    throw new RuntimeException("TODO");
  }

  private static <T> Object[] buildLeaf(BulkIterator<T> sortedBulk, int size) {

    Object[] vals = new Object[size | 1]; /* Leaf node must be odd. */
    sortedBulk.store(vals, 0, size);
    return vals;
  }

  private static <T> Object[] buildTree(BulkIterator<T> sortedBulk, int size) {

    int requireHeight = requireHeight(size);

    assert requireHeight > 1;
    assertHardHeight(requireHeight);

    throw new RuntimeException("TODO");
  }

  /**
   * Build a dense BTree from top to bottom with given height.
   * @childSize: the number of children required for the internal node at given height.
   * @tupleSize: the number of tuples stored in the tree with given height.
   * Caller is responsible to pass the correct childSize and height for the given tuples.
   */
  private static <T> Object[] denselyBuild(BulkIterator<T> sortedBulk, int childSize, int tupleSize, int height) {

    assert childSize <= MAX_TUPLES + 1;

    Object[] internal = new Object[childSize * 2];
    int childOffset = childSize - 1; /* descend start from second half of internal array. */

    /**
     * cutoff serves two purposes:
     *  1. guarantee there is enough element in the last child(last leaf or last sub-tree). (Make sure ODD children in the internal node.)
     *  2. make sure that last child is still likely balance.
     */
    if ( height == 2 ) {

      int remaining = tupleSize;
      int cutoff = MAX_TUPLES + 1 + MIN_TUPLES;

      int i = 0;
      while ( remaining >= cutoff ) {
        internal[childOffset + i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        remaining -= MAX_TUPLES + 1;
        i++;
      }
      if ( remaining > cutoff ) {
        int leafTupleSize = remaining/2;
        internal[childOffset+i] = buildLeaf(sortedBulk, leafTupleSize);
        remaining -= leafTupleSize + 1;
        i++;
      }
      internal[childOffset + i] = buildLeaf(sortedBulk, remaining);
      i++;

      assert i == childSize;
    } else {
      --height;
      int maxChildTupleSize = maxTreeSize(height);
      int maxGrandChildTupleSize = maxTreeSize(height-1);

      int remaining = tupleSize;
      int cutoff = maxChildTupleSize + 1 + MIN_TUPLES * (maxGrandChildTupleSize + 1);
      
      int i = 0;
      while ( remaining >= cutoff ) {
        internal[childOffset + i] = buildFullTree(sortedBulk, height);
        internal[i] = sortedBulk.next();
        remaining -= maxChildTupleSize + 1;
        i++;
      }

      if ( remaining > maxChildTupleSize ) {
        int grandChildSize = remaining / ((maxGrandChildTupleSize + 1) * 2); // == remaining / (maxGrandChildTupleSize + 1 ) / 2
        int grandChildTupleSize = grandChildSize * (maxGrandChildTupleSize+1) - 1;
        internal[childOffset + i] = denselyBuild(sortedBulk, grandChildSize, grandChildTupleSize, height);
        internal[i] = sortedBulk.next();
        remaining -= grandChildTupleSize + 1;
        i++;
      }

      int grandChildSize = remaining / (maxGrandChildTupleSize + 1) + 1;
      assert grandChildSize >= MIN_TUPLES + 1;
      int grandChildTupleSize = remaining;
      internal[childOffset + i] = denselyBuild(sortedBulk, grandChildSize, grandChildTupleSize, height);
      i++;

      assert i == childSize;
    }

    internal[childSize*2 - 1] = new ZoneMap();
    return internal;
  }

  /**
   * Build a full tree with given heigh.
   *  - require the sortedBulk has enough tuples to build a full tree.
   *  - full tree size == 1<<(height*fanout_shift) - 1
   */
  private static <T> Object[] buildFullTree(BulkIterator<T> sortedBulk, int height) {

    int descendStart = MAX_TUPLES; /* child reference begin at (FANOUT-1/MAX_TUPLES) position in the array */
    /**
     * full size node.
     *  - MAX_TUPLES keys + (MAX_TUPLES+1) descendants + ZoneMap.
     */
    Object[] internal = new Object[FANOUT*2];

    if ( height == 2 ) {
      int i = 0;
      while ( i < descendStart ) {
        internal[descendStart+i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        i++;
      }
      internal[descendStart + i] = buildLeaf(sortedBulk, MAX_TUPLES);
    } else {
      int i = 0;
      while ( i < descendStart ) {
        internal[descendStart+i] = buildFullTree(sortedBulk, height-1);
        internal[i] = sortedBulk.next();
        i++;
      }
      internal[descendStart + i] = buildFullTree(sortedBulk, height-1);
    }

    internal[FANOUT*2-1] = new ZoneMap();
    return internal;
  }

  private static <T> T find(Object[] node, T key, Comparator<? super T> comparator) {
    while ( true ) {
      int keyEndIdx = getKeyEnd(node);
      int i = Arrays.binarySearch((T[]) node, 0, keyEndIdx, key, comparator); /* find matched tuple in the key range. */

      if ( i >= 0 ) {
        return (T) node[i];
      }

      if ( isLeafNode(node) ) {
        return null;
      }

      i = -1 - i;
      node = (Object[]) node[keyEndIdx + i];
    }
  }

  private static <T> int findInNode(Object[] node, T key, Comparator<? super T> comparator) {
    int keyEndIdx = getKeyEnd(node);
    return Arrays.binarySearch((T[]) node, 0, keyEndIdx, key, comparator);
  }

  // public static Object[] updateLeaves(Object[] oldNode, Object[] newNode, Comparator<? super Compare> Comparator) {
  //   return null;
  // }

  /**
   * Hard limit of the tree:
   *  - The tree can support up to 2^(fanoutShift*height) elements.
   */
  private static void assertHardHeight(int height) {
    assertHardHeight(height, FANOUT_SHIFT);
  }
  /**
   * Hard limit of the tree:
   *  - The tree can support up to 2^(fanoutShift*height) elements.
   */
  private static void assertHardHeight(int height, int fanoutShift) {
    assert height * fanoutShift < 32;
  }

  private static int requireHeight(int size) {
    return requireHeight(size, FANOUT_SHIFT);
  }

  /**
   * Calculate the minimum require height of a full tree filled by given size.
   * The calculate is a good enough estimation.
   */
  private static int requireHeight(int size, int fanoutShift) {
    int  v = 64 - Long.numberOfLeadingZeros(size);
    return (fanoutShift - 1 +  v) / fanoutShift;
  }

  private static int maxTreeSize(int height) {
    return maxTreeSize(height, FANOUT_SHIFT);
  }

  /**
   * Caculate the number of key-value pairs in a full tree with given height and default fanout_shift.
   * The calculate is a good enough estimation.
   */
  private static int maxTreeSize(int height, int fanoutShift) {
    return ( 1 << ( height * fanoutShift ) ) - 1;
  }

  private static int getKeyEnd(Object[] node) {
    if ( isLeafNode(node) ) return getLeafKeyEnd(node);
    return getInternalKeyEnd(node);
  }

  private static int getLeafKeyEnd(Object[] node) {
    int length = node.length;
    return node[length-1] == null ? length - 1 : length; /* Leaf is made up to be odd, if it is even. */
  }

  private static int getInternalKeyEnd(Object[] node) {
    return (node.length / 2) - 1; /* internal node size must be even. */
  }

  private static boolean isLeafNode(Object[] node) {
    return ( node.length & 1 ) == 1;
  }

  private static int leafSize(Object[] leaf) {
    int length = leaf.length;
    return leaf[length-1] == null ? length - 1 : length;
  }

  private static class NodeBuilder {

    int height;
    int size;
    Object[] buffer;

    Object[] overflowBuffer;
  }
}