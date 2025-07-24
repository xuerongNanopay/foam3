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
 *  - Force the tree element to odd for better balance and code simplicity.
 *  - Max support elements size is: height * FANOUT_SHIFT < 32.
 */

public class BTree {

  //TODO: configure FANOUT_SHIFT.
  public static final int FANOUT_SHIFT = 5;
  private static final int FANOUT = 1 << FANOUT_SHIFT;
  public static final int MIN_NODE_KEYS = FANOUT / 2 - 1;
  public static final int MAX_NODE_KEYS = FANOUT - 1;

  private static final Object[] EMPTY_LEAF = new Object[1];

  public static Object[] empty() {
    return EMPTY_LEAF;
  }

  public static Object[] singleton(Object val) {
    return new Object[]{ val };
  }

  public static <V> Object[] build(BulkIterator<V> source, int size) {
    if ( size == 0 ) return EMPTY_LEAF;
    if ( size <= MAX_NODE_KEYS ) return buildLeaf(source, size);

    // return buildRoot();
    throw new RuntimeException("TODO");
  }

  private static <V> Object[] buildLeaf(BulkIterator<V> source, int size) {

    Object[] vals = new Object[size | 1]; /* force tree node element to be odd. */
    source.store(vals, 0, size);
    return vals;
  }

  private static <V> Object[] buildRoot(BulkIterator<V> source, int size) {
    int requireHeight = requireHeight(size);

    assert requireHeight > 1;
    assertHeight(requireHeight);

    throw new RuntimeException("TODO");
  }

  private static <V> Object[] buildMostlyFullTree(BulkIterator<V> source, int childSize, int size, int height) {

    assert childSize <= MAX_NODE_KEYS + 1;

    Object[] node = new Object[childSize * 2];

    if ( height == 2 ) {

      int remaining = size;
      int threshold = MAX_NODE_KEYS + 1 + MIN_NODE_KEYS;
      int i = 0;

      if ( remaining > MAX_NODE_KEYS ) {

      }
    }

    throw new RuntimeException("TODO");
  }

  private static Object[] build;

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

  private static int requireHeight(int size) {
    return requireHeight(size, FANOUT_SHIFT);
  }

  /**
   * Calculate the minimum requirement of full tree height.
   */
  private static int requireHeight(int size, int fanoutShift) {
    int  v = 64 - Long.numberOfLeadingZeros(size);
    return (fanoutShift - 1 +  v) / fanoutShift;
  }

  private static int fullTreeNode(int height) {
    return fullTreeNode(height, FANOUT_SHIFT);
  }

  /**
   * Caculate the number of nodes(internal+leaf) in a full tree with given height and default fanout_shift.
   */
  private static int fullTreeNode(int height, int fanoutShift) {
    return ( 1 << ( height * fanoutShift ) ) - 1;
  }

  /**
   * Calculate the minimum tree height required for given size.
   */
  private static int minHeight(int size) {
    return fullTreeHeight(size, FANOUT_SHIFT);
  }

  private static int fullTreeHeight(int size, int fanoutShift) {
    int v = 64 - Long.numberOfLeadingZeros(size);
    return (fanoutShift - 1 + v) / fanoutShift;
  }

  // private static Object[] buildLeaf(Collection source) {
  //   Object[] values = new Object[source.size() | 1]; // odd-length array.

    
  // }
}