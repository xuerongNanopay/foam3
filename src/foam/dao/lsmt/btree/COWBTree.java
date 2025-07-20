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
 */

public class COWBTree {

  //TODO: configure FANOUT_SHIFT.
  public static final int FANOUT_SHIFT = 5;
  private static final int FANOUT_FACTOR = 1 << FANOUT_SHIFT;
  public static final int MIN_NODE_KEYS = FANOUT_FACTOR / 2 - 1;
  public static final int MAX_NODE_KEYS = FANOUT_FACTOR - 1;

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
    int minHeight = minHeight(size);

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
   * Calculate the minimum tree height required for given size.
   */
  private static int minHeight(int size) {
    throw new RuntimeException("TODO");
  }

  private static int heightAtSize(int size, int fanoutShift) {
    int v = 64 - Long.numberOfLeadingZeros(size);
    return (fanoutShift - 1 + v) / fanoutShift;
  }

  // private static Object[] buildLeaf(Collection source) {
  //   Object[] values = new Object[source.size() | 1]; // odd-length array.

    
  // }
}