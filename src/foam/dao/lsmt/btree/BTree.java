/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.*;
import java.util.function.Function;

import foam.dao.lsmt.utils.BulkIterator;

/**
 * Copy-on-Write Btree.
 *  - No in-place update. Caller should use CAS for thread-safe.
 *  - Max support child size is limited by: tree_height * FANOUT_SHIFT < 32.
 *  - key and value doesn't not seperate, they are wrapped in the Object(We called it Tuple).
 *  - Tuple is comparable.
 *  - The full tree size is always ODD.
 *  - The entire btree is structed in Object array.
 *    - The leaf array must be ODD in length.(if the size of leaf is even, it will append extra NULL element at the end.)
 *    - The internal array must be EVEN in length. The last element in the internal array will be zoom map.
 *        - layout: [tuple1, tuple2, ... tupleN, child1, child2, .... childN+1, ZoomMap]
 *        - child is reference to sub tree(reference to Object Array).
 *        - algorithm will guarantee the child size is ODD and tuple size is child size - 1, in order to make sure the array length of internal node is EVEN.
 */

public class BTree {

  //TODO: configure FANOUT_SHIFT.
  public static final int FANOUT_SHIFT = 5;
  private static final int FANOUT = 1 << FANOUT_SHIFT;
  public static final int MIN_TUPLES = FANOUT / 2 - 1;
  public static final int MAX_TUPLES = FANOUT - 1;

  private static final Object[] EMPTY_LEAF = new Object[1];

  static final int[][] FULL_PRE_SUM_CACHE = fullPreSumCache(FANOUT_SHIFT);

  public static Object[] empty() {
    return EMPTY_LEAF;
  }

  public static Object[] singleton(Object val) {
    return new Object[]{ val };
  }

  public static <T> Object[] build(BulkIterator<T> sortedBulk, int size) {
    assert size >= 0;

    if ( size == 0 ) return EMPTY_LEAF;
    if ( size <= MAX_TUPLES ) return buildLeaf(sortedBulk, size);
    return buildInternal(sortedBulk, size);
  }

  private static <T> Object[] buildLeaf(BulkIterator<T> sortedBulk, int size) {

    Object[] vals = new Object[((int) size) | 1]; /* Leaf node must be odd. */
    sortedBulk.store(vals, 0, (int) size);
    return vals;
  }

  private static <T> Object[] buildInternal(BulkIterator<T> sortedBulk, int size) {

    int height = requireHeight(size);

    assert height > 1;
    assertHeight(height);

    int fullChildSize = fullTreeSize(height-1);
    int tupleSize = (int) (size / (fullChildSize + 1) + 1);

    return denselyBuild(sortedBulk, tupleSize, size, height);
  }

  /**
   * Build a dense BTree from top to bottom with given height.
   * @tupleSize: the number of children required for the current internal\.
   * @size: the number of tuples stored in the tree with given height from sortedBulk.
   * Caller is responsible to pass the correct childTupleSize and height for the given tuples.
   */
  private static <T> Object[] denselyBuild(BulkIterator<T> sortedBulk, int tupleSize, int size, int height) {

    assert tupleSize <= MAX_TUPLES + 1;

    Object[] internal = new Object[tupleSize * 2];
    int childOffset = tupleSize - 1; /* descend start from second half of internal array. */
    var presum = new int[tupleSize];

    if ( height == 2 ) {

      int remaining = size;
      int fullChildSizeCutOff = MAX_TUPLES + 1 + MIN_TUPLES; // make sure there are at lest one element in the last child.

      int i = 0;
      while ( remaining >= fullChildSizeCutOff ) {
        internal[childOffset + i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        remaining -= MAX_TUPLES + 1;
        presum[i++] = size - (remaining+1);
      }
      if ( remaining > MAX_TUPLES ) {
        int leafSize = remaining/2;
        internal[childOffset+i] = buildLeaf(sortedBulk, leafSize);
        internal[i] = sortedBulk.next();
        remaining -= leafSize + 1;
        presum[i++] = size - (remaining+1);
      }
      internal[childOffset + i] = buildLeaf(sortedBulk, remaining);
      presum[i++] = size;

      assert i == tupleSize;
    } else {
      int fullChildSize = fullTreeSize(--height);
      int fullGrandChildSize = fullTreeSize(height-1);

      int remaining = size;
      int fullChildSizeCutOff = fullChildSize + 1 + MIN_TUPLES * (fullGrandChildSize + 1);
      
      int i = 0;
      while ( remaining >= fullChildSizeCutOff ) {
        internal[childOffset + i] = fullyBuild(sortedBulk, height);
        internal[i] = sortedBulk.next();
        remaining -= fullChildSize + 1;
        presum[i++] = size - (remaining+1);
      }

      if ( remaining > fullChildSize ) {
        int grandChildTupleSize = remaining / ((fullGrandChildSize + 1) * 2); // remaingin / fullGrandChildSize + 1 / 2
        int grandChildSize = grandChildTupleSize * (fullGrandChildSize+1) - 1;
        internal[childOffset + i] = denselyBuild(sortedBulk, (int) grandChildTupleSize, grandChildSize, height);
        internal[i] = sortedBulk.next();
        remaining -= grandChildSize + 1;
        presum[i++] = size - (remaining+1);
      }

      int grandChildTupleSize = remaining / (fullGrandChildSize + 1) + 1;
      assert grandChildTupleSize >= MIN_TUPLES + 1;
      int grandChildSize = remaining;
      internal[childOffset + i] = denselyBuild(sortedBulk, (int) grandChildTupleSize, grandChildSize, height);
      presum[i++] = size;

      assert i == tupleSize;
    }

    internal[tupleSize*2 - 1] = presum;
    return internal;
  }

  /**
   * Build a full tree with given heigh.
   *  - require the sortedBulk has enough tuples to build a full tree.
   *  - full tree size == 1<<(height*fanout_shift) - 1
   */
  private static <T> Object[] fullyBuild(BulkIterator<T> sortedBulk, int height) {

    Object[] internal = new Object[FANOUT*2];

    if ( height == 2 ) {
      for ( int i = 0 ; i < MAX_TUPLES ; i++ ) {
        internal[MAX_TUPLES+i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
      }
      internal[MAX_TUPLES*2] = buildLeaf(sortedBulk, MAX_TUPLES);

    } else {

      for ( int i = 0 ; i < MAX_TUPLES ; i++ ) {
        internal[MAX_TUPLES+i] = fullyBuild(sortedBulk, height-1);
        internal[i] = sortedBulk.next();
      }
      internal[MAX_TUPLES*2] = fullyBuild(sortedBulk, height-1);
    }

    internal[MAX_TUPLES*2+1] = FULL_PRE_SUM_CACHE[height-2];
    return internal;
  }

  /**
   * to: excluded
   */
  static <C> int search(Comparator<? super C> comparator, Object[] node, int from , int to, C key) {
    return Arrays.binarySearch((C[])node, from, to, key, comparator);
  }

  /**
   * if key = null, then key is postive infinite.
   * to: excluded
   */
  static <C> int searchWithMaybePosiInfi(Comparator<? super C> comparator, Object[] node, int from , int to, C key) {
    if ( key == null ) return -(++to);
    return Arrays.binarySearch((C[])node, from, to, key, comparator);
  }

  /**
   * to: excluded
   */
  static <C> int searchWithUpperBound(Comparator<? super C> comparator, Object[] node,  int from, int to, C upperBound, C key) {

    int step = 0;
    while ( true ) {
      int i = from + step;
      if ( i >= to ) {
        int c = compareWithMaybePosiInfi(comparator, key, upperBound);
        if ( c >= 0 ) {
          // search key is greater or equal to upperBound.
          // so, the insert postion should be to+1.
          return -(2 + to);
        }
        break;
      }

      int c = comparator.compare(key, (C) node[i]);
      if ( c < 0 ) {
        to = i;
        break;
      }
      if ( c == 0 ) return i;

      from = i + 1;
      step = step * 2 + 1;
    }
    return Arrays.binarySearch((C[])node, from, to, key, comparator);
  }

  /**
   * if b = null, then b is postive infinite.
   */
  static <C> int compareWithMaybePosiInfi(Comparator<? super C> comparator, C a, C b) {
    if ( b == null ) return -1;
    return comparator.compare(a, b);
  }

  public static <T> T find(Comparator<? super T> comparator, Object[] node, T tuple) {
    while ( true ) {
      int tupleEnd = getTupleEnd(node);
      int i = Arrays.binarySearch((T[]) node, 0, tupleEnd, tuple, comparator); /* find matched tuple in the key range. */

      if ( i >= 0 ) {
        return (T) node[i];
      }

      if ( isLeaf(node) ) {
        return null;
      }

      i = -1 - i;
      node = (Object[]) node[tupleEnd + i];
    }
  }

  static <T> T findByInOrderIndex(Object[] node, int inOrderIdx) {

    assert inOrderIdx > 0 && inOrderIdx < size(node) 
      : inOrderIdx + " must be in the range [0, " + (size(node) - 1) + "]";


    while ( true ) {
      if ( isLeaf(node) ) {
        int tupleSize = tupleSizeOfLeaf(node);
        assert inOrderIdx < tupleSize;
        return (T) node[inOrderIdx];
      }

      int[] presum = getPresum(node);
      int find = Arrays.binarySearch(presum, inOrderIdx);
      if ( find >= 0 ) {
        assert find < presum.length - 1;
        return (T) node[find];
      }

      int descend = -find -1;
      
      assert descend < presum.length;
      inOrderIdx -= (descend == 0 ? 0 : presum[descend-1] + 1);

      node = (Object[]) node[firstChildOfInternal(node) + descend];
    }
  }

  static <T> void replaceInSitu(Object[] node, int inOrderIdx, T newTuple) {

    assert inOrderIdx > 0 && inOrderIdx < size(node) 
      : inOrderIdx + " must be in the range [0, " + (size(node) - 1) + "]";

    while ( ! isLeaf(node) ) {

      int[] presum = getPresum(node);
      int find = Arrays.binarySearch(presum, inOrderIdx);

      if ( find >= 0 ) {
        assert find < presum.length - 1;
        node[find] = newTuple;
        return;
      }

      int descend = -find - 1;
      assert descend < presum.length;
      inOrderIdx -= (descend == 0 ? 0 : presum[descend-1] + 1);
      node = (Object[]) node[firstChildOfInternal(node) + descend];
    }

    assert inOrderIdx < getLeafTupleEnd(node);
    node[inOrderIdx] = newTuple;
  }

  // private static <T> int findInNode(Object[] node, T key, Comparator<? super T> comparator) {
  //   int keyEndIdx = getTupleEnd(node);
  //   return Arrays.binarySearch((T[]) node, 0, keyEndIdx, key, comparator);
  // }

  /**
   * Hard limit of the tree:
   *  - The tree can support up to 2^(fanoutShift*height) elements.
   */
  private static void assertHeight(int height) {
    assertHeight(height, FANOUT_SHIFT);
  }
  /**
   * Hard limit of the tree:
   *  - The tree can support up to 2^(fanoutShift*height) elements.
   */
  private static void assertHeight(int height, int fanoutShift) {
    assert height * fanoutShift < 32;
  }

  static int requireHeight(int size) {
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

  private static int fullTreeSize(int height) {
    return fullTreeSize(height, FANOUT_SHIFT);
  }

  /**
   * Caculate the number of key-value pairs in a full tree with given height and default fanout_shift.
   * The calculate is a good enough estimation.
   */
  private static int fullTreeSize(int height, int fanoutShift) {
    return ( 1 << ( height * fanoutShift ) ) - 1;
  }

  static int getTupleEnd(Object[] node) {
    if ( isLeaf(node) ) return getLeafTupleEnd(node);
    return getInternalTupleEnd(node);
  }

  private static int getLeafTupleEnd(Object[] leaf) {
    int length = leaf.length;
    return leaf[length-1] == null ? length - 1 : length; /* Leaf is made up to be odd, if it is even. */
  }

  private static int getInternalTupleEnd(Object[] internal) {
    return (internal.length / 2) - 1; /* internal node size must be even. */
  }

  static int tupleSize(Object[] node) {
    if ( isLeaf(node) ) return getLeafTupleEnd(node);
    return getInternalTupleEnd(node);
  }

  static int tupleSizeOfLeaf(Object[] leaf) {
    return getLeafTupleEnd(leaf);
  }

  static int tupleSizeOfInternal(Object[] internal) {
    return getInternalTupleEnd(internal);
  }

  static int firstChildOfInternal(Object[] internal) {
    return getInternalTupleEnd(internal);
  }

  static int[] getPresum(Object[] internal) {
    return (int[]) internal[internal.length-1];
  }

  public static boolean isEmpty(Object[] tree) {
    return tree == EMPTY_LEAF;
  }

  public static boolean isLeaf(Object[] node) {
    return ( node.length & 1 ) == 1;
  }

  static int sizeOfLeaf(Object[] leaf) {
    int l = leaf.length;
    return leaf[l-1] == null ? l - 1 : l;
  }

  static int sizeOfInternal(Object[] internal) {
    int[] presum = getPresum(internal);
    return presum[presum.length-1];
  }

  /**
   * Leaf height is 1.
   */
  public static int height(Object[] tree) {

    int height = 1;
    while ( ! isLeaf(tree) ) {
      tree = (Object[]) tree[firstChildOfInternal(tree)];
      height++;
    }

    return height;
  }

  public static int size(Object[] tree) {
    if ( isLeaf(tree) ) return getLeafTupleEnd(tree);
    return sizeOfInternal(tree);
  }

  private static int[][] fullPreSumCache(int fanoutShift) {
    
    int height = 32/fanoutShift - 1; // Skip height == 1 (LEAF).
    int childSize = 1 << fanoutShift;
    int[][] presum = new int[height][childSize];

    for ( int i = 0 ; i < height ; i++ ) {
      int size = fullTreeSize(i+1);
      int aggSum = 0;

      for ( int j = 0 ; j < childSize ; j++ ) {
        presum[i][j] = aggSum += size;
        aggSum++;
      }
    }

    return presum;
  }

  /**
   * shadow compare two nodes
   */
  static boolean areNodeIdentical(Object[] a, int aOffset, Object[] b, int bOffset, int count) {
    for ( int i = 0 ; i < count ; i++ ) {
      if ( a[i+aOffset] != b[i+bOffset] ) return false;
    }
    return true;
  }

  /**
   * A utility method for shadow comparing a range of two Object arrays.
   * callar should guarantee not out-of-bound
   */
  static boolean areShadowIdentical(Object[] a, int aFrom, Object[] b, int bFrom, int size) {
    for ( int i = 0 ; i < size ; i++ ) 
      if ( a[aFrom + i] != b[bFrom + i] )
        return false;
    return true;
  }

}