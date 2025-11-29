/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import static foam.dao.lsmt.btree.BTree.*;

public class BTreeUpdate {

  static abstract class NodeBuilder {

    final int height;
    final NodeBuilder child;
    InternalBuilder parent;

    Object[] buffer;
    int count;

    Object[] readyBuffer; // cache precede node.
    Object splitTuple;

    NodeBuilder(NodeBuilder child) {
      this.child = child;
      this.height = child == null ? 1 : 1 + child.height;
    }

    final boolean hasOverflow() {
      return splitTuple != null;
    }

    final boolean isSufficient() {
      return count >= MIN_TUPLES || hasOverflow();
    }

    final boolean mustRebalance() {
      return count < MIN_TUPLES && hasOverflow();
    }

    final boolean isEmpty() {
      return count == 0 && !hasOverflow();
    }

    abstract void addTuple(Object tuple);

    abstract Object[] flush();
    abstract void flushToParent(InternalBuilder parentBuilder);

    final Object[] build() {
      NodeBuilder cur = this;

      while ( true ) {
        if ( !cur.hasOverflow() ) {
          return cur.flush();
        }

        InternalBuilder parent = parent();
        cur.flushToParent(parent);
        cur = parent;
      }
    }

    final InternalBuilder parent() {
      if ( parent == null ) parent = new InternalBuilder(this);
      return parent;
    }
  }

  static class LeafBuilder extends NodeBuilder {

    LeafBuilder() {
      super(null);
      this.buffer = new Object[MAX_TUPLES];
    }

    void addTuple(Object tuple) {
      if ( count == MAX_TUPLES ) {
        overflow(tuple);
      } else {
        buffer[count++] = tuple;
      }
    }

    void overflow(Object tuple) {

      if ( hasOverflow() ) {
        flushOverflow();
      }

      splitTuple = tuple;
      readyBuffer = buffer;
      buffer = new Object[MAX_TUPLES];
      count = 0;
    }

    void flushOverflow() {

    }

    Object[] flush() {
      throw new RuntimeException("TODO");
    }

    void flushToParent(InternalBuilder parentBuilder) {
      throw new RuntimeException("TODO");
    }
  }

  static class InternalBuilder extends NodeBuilder {



    InternalBuilder(NodeBuilder childBuilder) {
      super(null); //FIXME
    }

    void addTuple(Object tuple) {
      throw new RuntimeException("TODO");
    }

    void addChild(Object[] child, int childSize) {

    }

    void addChildAndTuple(Object[] child, int childSize, Object tuple) {
      addChild(child, childSize);
      addTuple(tuple);
    }

    Object[] flush() {
      throw new RuntimeException("TODO");
    }
    
    void flushToParent(InternalBuilder parentBuilder) {
      throw new RuntimeException("TODO");
    }
  }
}