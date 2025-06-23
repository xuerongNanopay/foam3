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
 */
public class COWBTree {

  private static final int FANOUT_FACTOR = 32;
  public static final int MIN_KEYS = FANOUT_FACTOR / 2 - 1;
  public static final int MAX_KEYS = FANOUT_FACTOR - 1;

  private static final Object[] EMPTY_LEAF = new Object[1];

  public static Object[] empty() {
    return EMPTY_LEAF;
  }

  public static Object[] singleton(Object val) {
    return new Object[]{ val };
  }

  public static <V> Object[] build(BulkIterator<V> source, int size) {
    if ( size == 0 ) return EMPTY_LEAF;
    if ( size <= MAX_KEYS ) return buildLeaf(source, size);

    // return buildRoot();
    return null;
  }

  private static <V> Object[] buildLeaf(BulkIterator<V> source, int dize) {
    return null;
  }

  // private static Object[] buildLeaf(Collection source) {
  //   Object[] values = new Object[source.size() | 1]; // odd-length array.

    
  // }
}