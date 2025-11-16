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

  public static Object[] empty() {
    return EMPTY_LEAF;
  }

  public static Object[] singleton(Object val) {
    return new Object[]{ val };
  }

  public static <T> Object[] build(BulkIterator<T> sortedBulk, long size) {
    assert size >= 0;

    if ( size == 0 ) return EMPTY_LEAF;
    if ( size <= MAX_TUPLES ) return buildLeaf(sortedBulk, size);
    return buildInternal(sortedBulk, size);
  }

  private static <T> Object[] buildLeaf(BulkIterator<T> sortedBulk, long size) {

    Object[] vals = new Object[((int) size) | 1]; /* Leaf node must be odd. */
    sortedBulk.store(vals, 0, (int) size);
    return vals;
  }

  private static <T> Object[] buildInternal(BulkIterator<T> sortedBulk, long size) {

    int height = requireHeight(size);

    assert height > 1;
    assertHeight(height);

    long fullChildSize = fullTreeSize(height-1);
    int tupleSize = (int) (size / (fullChildSize + 1) + 1);

    return denselyBuild(sortedBulk, tupleSize, size, height);
  }

  /**
   * Build a dense BTree from top to bottom with given height.
   * @tupleSize: the number of children required for the current internal\.
   * @size: the number of tuples stored in the tree with given height from sortedBulk.
   * Caller is responsible to pass the correct childTupleSize and height for the given tuples.
   */
  private static <T> Object[] denselyBuild(BulkIterator<T> sortedBulk, int tupleSize, long size, int height) {

    assert tupleSize <= MAX_TUPLES + 1;

    Object[] internal = new Object[tupleSize * 2];
    int childOffset = tupleSize - 1; /* descend start from second half of internal array. */
    var preSum = new long[tupleSize];
    /**
     * cutoff serves two purposes:
     *  1. guarantee there is enough element in the last child(last leaf or last sub-tree). (Make sure ODD children in the internal node.)
     *  2. make sure that last child is still likely balance.
     */
    if ( height == 2 ) {

      long remaining = size;
      long fullChildSizeCutOff = MAX_TUPLES + 1 + MIN_TUPLES;

      int i = 0;
      while ( remaining >= fullChildSizeCutOff ) {
        internal[childOffset + i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        remaining -= MAX_TUPLES + 1;
        preSum[i++] = size - (remaining+1);
      }
      if ( remaining > MAX_TUPLES ) {
        long leafSize = remaining/2;
        internal[childOffset+i] = buildLeaf(sortedBulk, leafSize);
        internal[i] = sortedBulk.next();
        remaining -= leafSize + 1;
        preSum[i++] = size - (remaining+1);
      }
      internal[childOffset + i] = buildLeaf(sortedBulk, remaining);
      preSum[i++] = size;

      assert i == tupleSize;
    } else {
      long fullChildSize = fullTreeSize(--height);
      long fullGrandChildSize = fullTreeSize(height-1);

      long remaining = size;
      long fullChildSizeCutOff = fullChildSize + 1 + MIN_TUPLES * (fullGrandChildSize + 1);
      
      int i = 0;
      while ( remaining >= fullChildSizeCutOff ) {
        internal[childOffset + i] = fullyBuild(sortedBulk, height);
        internal[i] = sortedBulk.next();
        remaining -= fullChildSize + 1;
        preSum[i++] = size - (remaining+1);
      }

      if ( remaining > fullChildSize ) {
        long grandChildTupleSize = remaining / ((fullGrandChildSize + 1) * 2); // remaingin / fullGrandChildSize + 1 / 2
        long grandChildSize = grandChildTupleSize * (fullGrandChildSize+1) - 1;
        internal[childOffset + i] = denselyBuild(sortedBulk, (int) grandChildTupleSize, grandChildSize, height);
        internal[i] = sortedBulk.next();
        remaining -= grandChildSize + 1;
        preSum[i++] = size - (remaining+1);
      }

      long grandChildTupleSize = remaining / (fullGrandChildSize + 1) + 1;
      assert grandChildTupleSize >= MIN_TUPLES + 1;
      long grandChildSize = remaining;
      internal[childOffset + i] = denselyBuild(sortedBulk, (int) grandChildTupleSize, grandChildSize, height);
      preSum[i++] = size;

      assert i == tupleSize;
    }

    internal[tupleSize*2 - 1] = new ZoneMap(preSum);
    return internal;
  }

  /**
   * Build a full tree with given heigh.
   *  - require the sortedBulk has enough tuples to build a full tree.
   *  - full tree size == 1<<(height*fanout_shift) - 1
   */
  private static <T> Object[] fullyBuild(BulkIterator<T> sortedBulk, int height) {

    int childStart = MAX_TUPLES; /* child reference begin at (FANOUT-1/MAX_TUPLES) position in the array */
    /**
     * full size node.
     *  - MAX_TUPLES keys + (MAX_TUPLES+1) descendants + ZoneMap.
     */
    Object[] internal = new Object[FANOUT*2];

    if ( height == 2 ) {
      int i = 0;
      while ( i < childStart ) {
        internal[childStart+i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        i++;
      }
      internal[childStart + i] = buildLeaf(sortedBulk, MAX_TUPLES);
    } else {
      int i = 0;
      while ( i < childStart ) {
        internal[childStart+i] = fullyBuild(sortedBulk, height-1);
        internal[i] = sortedBulk.next();
        i++;
      }
      internal[childStart + i] = fullyBuild(sortedBulk, height-1);
    }

    // internal[FANOUT*2-1] = new ZoneMap(preSum);
    return internal;
  }

  public static <C> Object[] update(Object[] oNode, Object[] nNode, Comparator<? super C> comparator) {

    if ( isEmpty(nNode) ) return oNode; // return old node if new node is empty.

    if ( isEmpty(oNode) ) return nNode; // return new node if old node is empty.


    // Both new and old nodes are leaf.
    if ( isLeaf(nNode) && isLeaf(oNode) ) {

    }

    return null;
  }

  public static <C, EXIST extends C, INSERT extends C> Object[] updateLeaves(Object[] existLeaf, Object[] insertLeaf, Comparator<? super C> comparator, TransformFunction<EXIST, INSERT> transform) {

    //TODO: merge oLeaf and nLeaf and sort user comparator.
    int existIdx = 0;
    int existSize = sizeOfLeaf(existLeaf);
    // EXIST existTuple = (EXIST) existLeaf[0];

    int insertIdx = 0;
    int insertSize = sizeOfLeaf(insertLeaf);
    // INSERT insertTuple = (INSERT) insertLeaf[0];

    //OPTIMIZE: skip elements in the existLeaf that are less and equal to the elements in the insertLeaf.

    try ( TreeBuilder<EXIST> builder = new TreeBuilder() ) {

      while ( existIdx < existSize && insertIdx < insertSize ) {
        EXIST existTuple = (EXIST) existLeaf[existIdx];
        INSERT insertTuple = (INSERT) insertLeaf[insertIdx];

        int c = comparator.compare(existTuple, insertTuple);
        if ( c == 0 ) {
          builder.add(transform.merge(existTuple, insertTuple));
          existTuple = (EXIST) existLeaf[++existIdx];
          insertTuple = (INSERT) insertLeaf[++insertIdx];
        } else if ( c < 0 ) {
          builder.add(existTuple);
          existTuple = (EXIST) existLeaf[++existIdx];
        } else {
          builder.add(transform.insert(insertTuple));
          insertTuple = (INSERT) insertLeaf[++insertIdx];
        }
      }

      if ( existIdx < existSize ) {
        builder.add((EXIST[]) existLeaf, existIdx, existSize-existIdx);
      } else if ( insertIdx < insertSize ) {
        builder.add((INSERT[]) insertLeaf, insertIdx,insertSize-insertIdx, transform);
      }
    }

    return null;
  }

  public static <C, O extends C, N extends C> List mergeLeaves(Object[] oLeaf, Object[] nLeaf, Comparator<? super C> comparator) {

    int oSize = sizeOfLeaf(oLeaf);
    int nSize = sizeOfLeaf(nLeaf);

    int oi = 0;
    int ni = 0;

    var tmp = new ArrayList();

    while ( oi < oSize && ni < nSize ) {
      int c = comparator.compare((O) oLeaf[oi], (C) nLeaf[ni]);

      if ( c < 0) {
        tmp.add(oLeaf[oi++]);
      } else if ( c == 0 ) {
        tmp.add(nLeaf[ni++]);
        oi++;
      } else {
        tmp.add(nLeaf[ni++]);
      }
    }

    while ( oi < oSize ) tmp.add(oLeaf[oi++]);
    while ( ni < nSize ) tmp.add(nLeaf[ni++]);

    
    return null;
  }

  private static <C> int search(Comparator<? super C> comparator, Object[] node, int from , int to, C key) {
    return Arrays.binarySearch((C[])node, from, to, key, comparator);
  }

  private static <T> T find(Object[] node, T key, Comparator<? super T> comparator) {
    while ( true ) {
      int keyEndIdx = getTupleEnd(node);
      int i = Arrays.binarySearch((T[]) node, 0, keyEndIdx, key, comparator); /* find matched tuple in the key range. */

      if ( i >= 0 ) {
        return (T) node[i];
      }

      if ( isLeaf(node) ) {
        return null;
      }

      i = -1 - i;
      node = (Object[]) node[keyEndIdx + i];
    }
  }

  private static <T> int findInNode(Object[] node, T key, Comparator<? super T> comparator) {
    int keyEndIdx = getTupleEnd(node);
    return Arrays.binarySearch((T[]) node, 0, keyEndIdx, key, comparator);
  }

  // public static Object[] updateLeaves(Object[] oldNode, Object[] newNode, Comparator<? super Compare> Comparator) {
  //   return null;
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

  private static int requireHeight(long size) {
    return requireHeight(size, FANOUT_SHIFT);
  }

  /**
   * Calculate the minimum require height of a full tree filled by given size.
   * The calculate is a good enough estimation.
   */
  private static int requireHeight(long size, int fanoutShift) {
    int  v = 64 - Long.numberOfLeadingZeros(size);
    return (fanoutShift - 1 +  v) / fanoutShift;
  }

  private static long fullTreeSize(int height) {
    return fullTreeSize(height, FANOUT_SHIFT);
  }

  /**
   * Caculate the number of key-value pairs in a full tree with given height and default fanout_shift.
   * The calculate is a good enough estimation.
   */
  private static long fullTreeSize(int height, int fanoutShift) {
    return ( 1L << ( height * fanoutShift ) ) - 1;
  }

  private static int getTupleEnd(Object[] node) {
    if ( isLeaf(node) ) return getLeafTupleEnd(node);
    return getInternalTupleEnd(node);
  }

  private static int getLeafTupleEnd(Object[] leaf) {
    int length = leaf.length;
    return leaf[length-1] == null ? length - 1 : length; /* Leaf is made up to be odd, if it is even. */
  }

  static int leafTupleSize(Object[] internal) {
    return getLeafTupleEnd(internal);
  }

  /**
   * Exclude end index.
   */
  private static int getInternalTupleEnd(Object[] internal) {
    return (internal.length / 2) - 1; /* internal node size must be even. */
  }

  static int firstChildOfInternal(Object[] internal) {
    return getInternalTupleEnd(internal);
  }

  static int intervalTupleSize(Object[] internal) {
    return getInternalTupleEnd(internal);
  }

  static ZoneMap getZoneMap(Object[] internal) {
    return (ZoneMap) internal[internal.length-1];
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

  static long sizeOfInternal(Object[] internal) {
    return getZoneMap(internal).size();
  }

  /**
   * Leaf height is 1.
   */
  public static int height(Object[] tree) {
    if ( isLeaf(tree) ) return 1;

    int height = 1;
    while ( ! isLeaf(tree) ) {
      tree = (Object[]) tree[firstChildOfInternal(tree)];
      height++;
    }

    return height;
  }

  public static long size(Object[] tree) {
    if ( isLeaf(tree) ) return getLeafTupleEnd(tree);
    return sizeOfInternal(tree);
  }

  private static abstract class NodeBuilder {

    final int height;
    final NodeBuilder child;
    int bufferCount;
    Object[] buffer;
    Object[] overflowBuffer;
    Object overflowLastTuple;

    NodeBuilder(NodeBuilder child) {
      this.height = child == null ? 1 : child.height;
      this.child = child;
    }

    final boolean hasOverflow() {
      return overflowLastTuple != null;
    }

    /**
     * A utility method for shadow comparing a range of two Object arrays.
     */
    static boolean areShadowIdentical(Object[] a, int aFrom, Object[] b, int bFrom, int size) {
      for ( int i = 0 ; i < size ; i++ ) 
        if ( a[aFrom + i] != b[bFrom + i] )
          return false;
      return true;
    }

    /**
     * A utility method for shadow comparing a range of two Object arrays.
     */
    static boolean areShadowIdentical(Object[] a, Object[] b, int from, int size) {
      return areShadowIdentical(a, from, b, from, size);
    }

    /**
     * A utility method for comparing a range of two int arrays.
     */
    static boolean areIdentical(int[] a, int aFrom, int[] b, int bFrom, int size) {
      for ( int i = 0 ; i < size ; i++ ) 
        if ( a[aFrom + i] != b[bFrom + i] )
          return false;
      return true;
    }

    /**
     * A utility method for comparing a range of two int arrays.
     */
    static boolean areIdentical(int[] a, int[] b, int from, int size) {
      return areIdentical(a, from, b, from, size);
    }
  }

  private static class LeafBuilder extends NodeBuilder {

    LeafBuilder() {
      super(null);
      this.buffer = new Object[MAX_TUPLES];
    }

    final void addTuple(Object newTuple) {
      if ( bufferCount == MAX_TUPLES ) {
        overflow(newTuple);
      } else {
        buffer[bufferCount++] = newTuple;
      }
    }

    void overflow(Object newTuple) {
      if ( hasOverflow() ){
        propagateOverflow();
      }

      overflowBuffer = buffer;
      overflowLastTuple = newTuple;
      
      buffer = new Object[MAX_TUPLES];
      bufferCount = 0;
    }

    void propagateOverflow() {

    }

    Object[] build() {
      return null;
    }

    Object[] flush() {
      throw new RuntimeException("TODO: flush implement");
    }
  }

  private static class InternalBuilder extends NodeBuilder {

    InternalBuilder(NodeBuilder child) {
      super(child);
      buffer = new Object[2 * (MAX_TUPLES + 1)];
    }


  }

  private static class TreeBuilder<E> extends LeafBuilder implements AutoCloseable {

    TreeBuilder() {

    }

    final LeafBuilder leaf() {
      return this;
    }

    void add(E tuple) {
      leaf().addTuple(tuple);
    }

    void add(E[] tuples, int offset, int size) {
      //TODO: add copy method on LeafBuilder.
      for ( int i = 0 ; i < size ; i++ ) {
        leaf().addTuple(tuples[offset+i]);
      }
    }

    <I> void add(I[] tuples, int offset, int size, TransformFunction<E, I> transfer) {
      for ( int i = 0 ; i < size ; i++ ) {
        leaf().addTuple(transfer.insert(tuples[offset+i]));
      }
    }

    Object[] build() {
      return leaf().build();
    }

    @Override
    public void close() {

    }
  }

  public interface TransformFunction<EXIST, INSERT>
  {
    EXIST insert(INSERT insertTuple);
    EXIST merge(EXIST existTuple, INSERT insertTuple);
  }
}