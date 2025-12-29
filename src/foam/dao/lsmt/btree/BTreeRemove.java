/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.Arrays;
import java.util.Comparator;
import static foam.dao.lsmt.btree.BTree.*;

public class BTreeRemove {

  public static <T> Object[] remove(Comparator<? super T> comparator, Object[] btree, T tuple) {

    if ( isEmpty(btree) ) {
      return btree;
    }

    int treeIndex = -1;
    int lSize = 0;
    T requireSwap = null;
    var node = btree;

    // Finding position where the tuple should be removed in BTree, assign to treeIndex.
    while ( true ) {
      int tupleSize = getTupleEnd(btree);
      // Casting to T is ok here, as it only touches tuples. 
      int find = Arrays.binarySearch((T[]) node, 0, tupleSize, tuple, comparator);

      if ( find >= 0 ) {
        // tuple found in current node.
        if ( isLeaf(node) ) {
          treeIndex = lSize + find;
        } else {
          int presum = getPresum(node)[find];
          treeIndex = lSize + presum - 1;
          //TODO: fix.
          requireSwap = (T) node[find];
        }
        break;
      }

      // Tuple no found in the btree, so return the original tree.
      if ( isLeaf(node) ) return btree;

      find = -(find + 1);
      if ( find > 0 ) lSize += getPresum(node)[find - 1] + 1;

      node = (Object[]) node[tupleSize + find];
    }

    if ( size(btree) == 1 ) return empty();

    return null;
    // if ( requireSwap == null ) {
    //   return removeFromLeaf();
    // } else {
    //   throw new RuntimeException("TODO: BTree remove");
    // }
  }

  /**
   * inOrderIndex must be between 0 to size(btree) - 1.
   */
  private static Object[] removeTuple(Object[] btree, int inOrderIndex) {

    Object[] node = btree;
    Object[] parent = null;

    while ( ! isLeaf(node) ) {
      break;
    }

    int tupleSize = sizeOfLeaf(node);
    Object[] newLeaf = new Object[(tupleSize & 1) == 1 ? tupleSize : tupleSize - 1];

    return null;
  }

  /**
   * @return: ending position after copy done.
   */
  private static int copyTuples(Object[] from, Object[] to, int offset, int skip) {
    int tupleSize = getTupleEnd(from);
    if ( skip > 0 ) {
      System.arraycopy(from, 0, to, offset, skip);
    }
    if ( skip + 1 < tupleSize ) {
      System.arraycopy(from, skip + 1, to, offset + skip, tupleSize - skip - 1);
    }
    return offset + tupleSize - 1;
  }

  /**
   * @return: ending position after copy done.
   */
  private static int copyTuples(Object[] from, Object[] to, int offset) {
    int tupleSize = getTupleEnd(from);
    System.arraycopy(from, 0, to, offset, tupleSize);
    return offset + tupleSize;
  }

  /**
   * @return: ending position after copy done.
   */
  private static int copyChildren(Object[] from, Object[] to, int offset, int skip) {

    assert !isLeaf(from);
    int childStart = tupleSizeOfInternal(from);
    int childSize = childStart + 1;

    if ( skip > 0 ) {
      System.arraycopy(from, childStart, to, offset, skip);
    }

    if ( skip + 1 <= childSize ) {
      System.arraycopy(from, childStart + skip + 1, to, offset + skip, childSize - skip - 1);
    }

    return offset + childSize - 1;
  }


  /**
   * @return: ending position after copy done.
   */
  // private static int copyChildren(Object[] from, Object[] to, int offset)v{

  //   assert !isLeaf(from);

  // }
}