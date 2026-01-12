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

  /**
   * Remove tuple from btree.
   * If the tuple is insde a leaf node, directly remove from leaf.
   * Otherwise, swap with tuple that is just less than it, and then delete tuple from leaf.
   */
  public static <T> Object[] remove(Comparator<? super T> comparator, Object[] btree, T tuple) {

    if ( isEmpty(btree) ) {
      return btree;
    }

    int removeIdx = -1;
    int accSum = 0;
    T swapTuple = null;
    var node = btree;

    // Finding position where the tuple should be removed in BTree, assign to removeIdx.
    while ( true ) {
      int tupleSize = getTupleEnd(btree);
      // Casting to T is ok here, as it only searchs in tuples. 
      int find = Arrays.binarySearch((T[]) node, 0, tupleSize, tuple, comparator);

      if ( find >= 0 ) {
        if ( isLeaf(node) ) {
          /**
           * Removing tuple is in leaf node.
           */
          removeIdx = accSum + find;
        } else {
          /**
           * Removing tuple is in internal node.
           */
          int nodePresum = getPresum(node)[find];
          removeIdx = accSum + nodePresum - 1;
          //TODO: add moethod to get last tuple directly.
          swapTuple = findByInOrderIndex(node, nodePresum - 1);
        }
        break;
      }

      if ( isLeaf(node) ) return btree;

      find = -find - 1; // convert to insert position.
      if ( find > 0 ) accSum += getPresum(node)[find - 1] + 1;

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
   * inOrderIdx: must be the in-order index for the tuple in the leaf.
   */
  private static Object[] removeTupleInLeaf(Object[] tree, int inOrderIdx) {

    Object[] newTree = null;
    Object[] node = tree;

    boolean requireClone = true;

    while ( ! isLeaf(node) ) {

      int tupleSize = tupleSize(node);
      int[] presum = getPresum(node);
      int insertP = Arrays.binarySearch(presum, inOrderIdx);
      assert insertP < 0; // caller guarantee inOrderIdx in the index for tuple in the leaf.
      insertP = -1 - insertP;
      if ( insertP > 0 ) {
        inOrderIdx -= presum[insertP - 1] + 1;
      }

      Object[] descendNode = (Object[]) node[tupleSize + insertP];
      boolean descendNodeRequireClone = true;

      if ( tupleSize(descendNode) > MIN_TUPLES ) {
        // delete tuple is inside descendTree
        // descendNode has enough tuples to ensure at lest min_tuples.
        node = needClone(node, requireClone);
      } else if ( insertP > 0 && tupleSize((Object[]) node[tupleSize + insertP - 1]) > MIN_TUPLES ) {
        // delete tuple is inside descendTree
        // descendNode does not have eough node, then steal from left sibling.
        node = needClone(node, requireClone);
        Object[] leftSibling = (Object[]) node[tupleSize + insertP -1];

        // add back steal size.
        ++inOrderIdx;
        if ( ! isLeaf(leftSibling) ) {
          inOrderIdx += size((Object[]) leftSibling[leftSibling.length - 2]);
        }

        descendNode = stealFromLeftInSitu(node, insertP);
      } else if ( insertP < tupleSize && tupleSize((Object[]) node[tupleSize + insertP + 1]) > MIN_TUPLES ) {
        // delete tuple is inside descendTree
        // descendNode does not have eough node, then steal from right sibling.
        node = needClone(node, requireClone);
        descendNode = stealFromRightInSitu(node, insertP);

      } else {
        descendNodeRequireClone = false;
        if ( insertP > 0 ) {
          // merge with left sibling.
          Object[] leftSibling = (Object[]) node[tupleSize + insertP - 1];
          Object sperateTuple = node[insertP - 1];
          
          // node = tupleSize ;

        } else {
          // merge with right sibling.
        }
      }

  
      break;
    }

    return newTree;
  }

  /**
   * In-place node mutation.
   * Clone a new internal before call this method.
   * idx: [1, childSize - 1]
   */
  private static Object[] stealFromRightInSitu(Object[] internal, int idx) {

    int childOffset = firstChildOfInternal(internal);
    Object[] lNode = (Object[]) internal[childOffset + idx];
    Object[] rNode = (Object[]) internal[childOffset + idx + 1];
    boolean isLeaf = isLeaf(lNode);
    int lSize = getTupleEnd(lNode);

    // Steal most left tuple/child from right node.
    Object[] stealChild = isLeaf ? null : (Object[]) rNode[firstChildOfInternal(rNode)];
    Object[] newLNode = insertTupleOrChild(lNode, lSize, internal[idx], lSize + 1, stealChild);

    // update internal in-place.
    internal[idx] = rNode[0];

    // Remove most left fhild from right node.
    internal[childOffset + idx + 1] = removeTupleOrChild(rNode, 0, 0, true);

    //FIXME: investigate why lNode doesn't need to assign.


    // update internal presum.
    getPresum(internal)[idx] += isLeaf ? 1 : 1 + size((Object[]) newLNode[newLNode.length - 2]);
    
    return newLNode;
  }

  /**
   * In-place node mutation.
   * Clone a new internal before call this method.
   */
  private static Object[] stealFromLeftInSitu(Object[] internal, int idx) {

    int childOffset = firstChildOfInternal(internal);
    Object[] rNode = (Object[]) internal[childOffset + idx];
    Object[] lNode = (Object[]) internal[childOffset + idx - 1];
    boolean isLeaf = isLeaf(rNode);
    int lSize = getTupleEnd(lNode);

    Object[] stealChild = isLeaf ? null : (Object[]) lNode[lNode.length - 2];
    Object[] newRNode = insertTupleOrChild(rNode, 0, internal[idx-1], 0, stealChild);

    internal[idx - 1] = lNode[lSize - 1];

    internal[childOffset + idx - 1] = removeTupleOrChild(lNode, lSize - 1, lSize, true);
    getPresum(internal)[idx - 1] -= isLeaf ? 1 : 1 + getPresum(newRNode)[0];

    return newRNode;
  }

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

  private static Object[] removeTupleOrChild(Object[] node, int tupleRemoveIdx, int childRemoveIdx, boolean adjustPresum) {

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
        retPresum[i-1] = adjustPresum ? presum[i] - removeSize : presum[i];
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

  private static Object[] needClone(Object[] node, boolean need) {

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