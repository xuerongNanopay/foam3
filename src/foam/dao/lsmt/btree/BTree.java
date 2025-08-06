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
 *  - Functional programing:
 *      1. No in place update.
 *      2. Mutation will clone from leaf to root. CAS on the root node.
 *  - Max support tuple size is: height * FANOUT_SHIFT < 32.
 *  - Reading are thread safe.
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

  private static <T> Object[] buildRoot(BulkIterator<T> sortedBulk, int size) {

    int requireHeight = requireHeight(size);

    assert requireHeight > 1;
    assertHardHeight(requireHeight);

    throw new RuntimeException("TODO");
  }

  /**
   * Build a full balance BTree from input.
   */
  private static <T> Object[] buildDenseTree(BulkIterator<T> sortedBulk, int internalSize, int size, int height) {

    assert internalSize <= MAX_TUPLES + 1;

    /**
     * Internal node must be even
     * [0 ... internalSize-1]: store tuples, also serve as key(post) for the internal node.
     * [internalSize ... internalSize*2-2]: store pointerto the descend nodes.
     * [internalSize*2-1]: ZoneMap.
     */
    Object[] internal = new Object[internalSize * 2];
    int descentStart = internalSize - 1; /* descent start from second half of internal array. */

    if ( height == 2 ) {

      int remaining = size;
      int i = 0;

      while ( remaining >= MAX_TUPLES ) {
        internal[descentStart + i] = buildLeaf(sortedBulk, MAX_TUPLES);
        internal[i] = sortedBulk.next();
        i++;
      }
      internal[descentStart + i] = buildLeaf(sortedBulk, MAX_TUPLES);
      i++;
      assert i == internalSize;
    } else {

    }

    throw new RuntimeException("TODO");
  }

  private static Object[] build;

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
}