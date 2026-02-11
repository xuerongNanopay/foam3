/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.util.btree;

import java.util.Arrays;
import java.util.Comparator;
import java.util.NoSuchElementException;

import static foam.dao.util.btree.BTree.*;

public class BTreeInternalCursor<K, T> {
  
  class NodeIter<K> {

    final NodeIter<K> parent;
    final NodeIter<K> child;
    final Comparator<? super K> comparator;

    Object[] node;
    int nodeOffset;
    int curIdx;
    boolean isInChild;

    NodeIter(Comparator<? super K> comparator, Object[] tree, NodeIter<K> parent) {

      this.comparator = comparator;
      this.parent = parent;
      this.node = tree;

      // Create NodeIter stack till the leaf.
      this.child = BTree.isLeaf(tree) ? null : new NodeIter<>(comparator, (Object[]) tree[firstChildOfInternal(tree)], this);
    }

    void resetNode(Object[] node, int nodeOffet) {
      node = node;
      nodeOffset = nodeOffset;
    }

    void seek(int inOrderIndex) {

      
    }

    boolean isLeaf() {
      return child == null;
    }

    int inOrderIndex () {
      return nodeOffset + BTree.inOrderIndex(node, curIdx);
    }
  }
}