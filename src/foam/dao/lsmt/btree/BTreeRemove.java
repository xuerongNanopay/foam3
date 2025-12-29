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
  // private static Object[] removeTuple(Object[] btree, int inOrderIndex) {

  //   Object[] node = btree;
  //   Object[] parent = null;

  //   while ( ! isLeaf(node) ) {
  //     break;
  //   }

  //   int tupleSize = sizeOfLeaf(node);
  //   Object[] newLeaf = new Object[(tupleSize & 1) == 1 ? tupleSize : tupleSize - 1];

  //   return null;
  // }

  /**
   * node must be a internal node.
   */
  // private static Object[] rotateRight(Object[] node, int childIdx) {

  //   int childOffset = firstChildOfInternal(node);
  //   Object[] child = (Object[]) node[childOffset + childIdx];
  //   Object[] rightSibling = (Object[]) node[childOffset + childIdx + 1];

  //   boolean isLeafChild = isLeaf(child);
  //   int childTupleEnd = getTupleEnd(child);
  //   Object[] newChild = isLeafChild ? null : (Object[]) rightSibling[]

  // }

  /**
   * make a new node with tuple and child inserting into given index.
   * childInsertIdx: base on 0 ofsset, not tuple size.
   */
  private static Object[] insertTupleOrChild(Object[] node, int tupleInsertIdx, Object tuple, int childInsertIdx, Object[] child) {

    boolean isLeaf = isLeaf(node);
    int tupleSize = getTupleEnd(node);

    Object[] ret;

    if ( isLeaf ) {
      // ensure leaf length is odd.
      ret = new Object[tupleSize + ((tupleSize & 1) == 1 ? 2 : 1)];
    } else {
      ret = new Object[node.length + 2];
    }

    if ( tupleInsertIdx > 0 ) {
      System.arraycopy(node, 0, ret, 0, tupleInsertIdx);
    }
    if ( tupleInsertIdx < tupleSize ) {
      System.arraycopy(node, tupleInsertIdx, ret, tupleInsertIdx + 1, tupleSize - tupleInsertIdx);
    }
    ret[tupleInsertIdx] = tuple;

    // Insert child.
    if ( ! isLeaf ) {

      if ( childInsertIdx > 0 ) {
        System.arraycopy(node, tupleSize, ret, tupleSize + 1, childInsertIdx);
      }
      if ( childInsertIdx <= tupleSize ) {
        System.arraycopy(node, tupleSize + childInsertIdx, ret, tupleSize + childInsertIdx + 2, tupleSize - childInsertIdx + 1);
      }
      ret[tupleSize + 1 + childInsertIdx] = child;

      int[] presum = getPresum(node);
      int[] retPresum = new int[presum.length + 1];

      if ( childInsertIdx > 0 ) {
        System.arraycopy(presum, 0, retPresum, 0, childInsertIdx);
      }
      int childSize = size(child);

      // calculate new presum.
      retPresum[childInsertIdx] = childSize + (childInsertIdx == 0 ? 0 : retPresum[childInsertIdx - 1] + 1);
      for ( int i = childInsertIdx + 1 ; i < retPresum.length ; ++i ) {
        retPresum[i] = presum[i - 1] + childSize + 1;
      }
      ret[ret.length - 1] = retPresum;
    }

    return ret;
  }

  private static Object[] removeTupleOrChild(Object[] node, int tupleRemoveIdx, int childRemoveIdx) {

    boolean isLeaf = isLeaf(node);
    int tupleSize = getTupleEnd(node);
    Object[] ret;

    if ( isLeaf ) {
      ret = new Object[tupleSize + ((tupleSize & 1) == 1 ? 0 : 1)];
    } else {
      ret = new Object[node.length - 2];
    }

    if ( tupleRemoveIdx > 0 ) {
      System.arraycopy(node, 0, ret, 0, tupleRemoveIdx);
    }
    if ( tupleRemoveIdx + 1 < tupleSize ) {
      System.arraycopy(node, tupleRemoveIdx + 1, ret, tupleRemoveIdx, tupleSize - tupleRemoveIdx - 1);
    }

    if ( ! isLeaf ) {

      if ( childRemoveIdx > 0 ) {
        System.arraycopy(node, tupleSize, ret, tupleSize - 1, childRemoveIdx);
      }
      if ( childRemoveIdx + 1 <= tupleSize ) {
        System.arraycopy(node, tupleSize + childRemoveIdx + 1, ret, tupleSize - 1 + childRemoveIdx, tupleSize - childRemoveIdx);
      }

      int[] presum = getPresum(node);
      int[] retPresum = new int[presum.length - 1];

      int removeSize = size((Object[]) node[firstChildOfInternal(node) + childRemoveIdx]) + 1;
      
      if ( childRemoveIdx > 0 ) {
        System.arraycopy(presum, 0, retPresum, 0, childRemoveIdx);
      }
      for ( int i = childRemoveIdx + 1 ; i < retPresum.length ; ++i ) {
        retPresum[i] = presum[i + 1] - removeSize;
      }
      ret[ret.length - 1] = retPresum;
    }

    return ret;
  }

  /**
   * merge two trees.
   */
  private static Object[] mergeMinNode(Object[] left, Object[] right, Object midTuple) {

    assert getTupleEnd(left) == MIN_TUPLES;
    assert getTupleEnd(right) == MIN_TUPLES;

    boolean isLeaf = isLeaf(left);
    Object[] ret;

    if ( isLeaf ) {
      ret = new Object[MIN_TUPLES * 2 + 1];
    } else {
      ret = new Object[left.length + right.length];
    }
    int offset = 0;
    offset = copyTuples(left, ret, offset);
    ret[offset++] = midTuple;
    offset = copyTuples(right, ret, offset);

    if ( ! isLeaf ) {
      offset = copyChildren(left, ret, offset);
      offset = copyChildren(right, ret, offset);
      int[] lPresum = getPresum(left);
      int[] rPresum = getPresum(right);
      int[] retPresum = new int[lPresum.length + rPresum.length];
      offset = 0;
      offset = copyPresum(lPresum, retPresum, offset, 0);
      offset = copyPresum(rPresum, retPresum, offset, lPresum[lPresum.length-1] + 1);
      ret[ret.length-1] = retPresum;
    }

    return ret;
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
  private static int copyChildren(Object[] from, Object[] to, int offset) {

    assert !isLeaf(from);

    int childStart = tupleSizeOfInternal(from);
    int childSize = childStart + 1;
    System.arraycopy(from, childStart, to, offset, childSize);

    return offset + childSize;
  }

  private static int copyPresum(int[] from, int[] to, int offset, int addition) {

    for ( int i = 0 ; i < from.length ; i++ ) {
      to[offset + i] = from[i] + addition;
    }
    return offset + from.length;
  }

  private static Object[] maybeCopy(Object[] node, boolean need) {

    if ( ! need ) return node;

    Object[] newNode = new Object[node.length];
    System.arraycopy(node, 0, newNode, 0, node.length);
    if ( ! isLeaf(node) ) {
      int[] presum = getPresum(node);
      int[] newPresum = new int[presum.length];
      System.arraycopy(presum, 0, newPresum, 0, presum.length);
      newNode[newNode.length-1] = newPresum;
    }
    return newNode;
  }

}