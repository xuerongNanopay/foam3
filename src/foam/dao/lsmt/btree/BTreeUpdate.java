/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.Arrays;
import static foam.dao.lsmt.btree.BTree.*;

public class BTreeUpdate {

  static abstract class NodeBuilder {

    final int height;
    final NodeBuilder child;
    InternalBuilder parent;

    Object[] origin;
    Object[] buffer;
    int count;

    Object[] overflowBuffer; // cache precede node.
    Object guideTuple;

    NodeBuilder(NodeBuilder child) {
      this.child = child;
      this.height = child == null ? 1 : 1 + child.height;
    }

    final boolean hasOverflow() {
      return guideTuple != null;
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

      guideTuple = tuple;
      overflowBuffer = buffer;
      buffer = new Object[MAX_TUPLES];
      count = 0;
    }

    void flushOverflow() {
      parent().addChildAndTuple(overflowBuffer, MAX_TUPLES, guideTuple);
      overflowBuffer = null;
      guideTuple = null;
    }

    Object[] flush() {

      assert !hasOverflow();

      if ( count == 0 ) return empty();

      Object[] leaf = new Object[count|1];
      System.arraycopy(buffer, 0 , leaf, 0, count);
      count = 0;
      return leaf;
    }

    void flushToParent(InternalBuilder parentBuilder) {
      Object[] leaf;
      int leafSize;

      if ( mustRebalance() ) {
        // steel tuples from overflow buffer and guide tuple to make current buffer reach to MIN_TUPLES.
        leafSize = MIN_TUPLES;
        leaf = new Object[MIN_TUPLES];

        int diff = MIN_TUPLES - count;
        System.arraycopy(overflowBuffer, MAX_TUPLES - diff - 1, leaf, 0, diff - 1);
        leaf[diff-1] = guideTuple;
        System.arraycopy(buffer, 0, leaf, diff, count);

        // Adjust overflow buffer and guide tuple.
        int predecessorRemaining = MAX_TUPLES - diff;
        Object[] predecessor = new Object[predecessorRemaining | 1];
        System.arraycopy(overflowBuffer, 0, predecessor, 0, predecessorRemaining);
        parent().addChildAndTuple(predecessor, predecessorRemaining, overflowBuffer[predecessorRemaining]);

        guideTuple = null;
      } else {
        if ( hasOverflow() ) {
          flushOverflow();
        }

        leafSize = count;
        leaf = flush();
      }
      count = 0;
      parentBuilder.addChild(leaf, leafSize); //TODO: need parentBuilder? or just parent()
    }

    void copy(Object[] leaf) {
      copy(leaf, 0, sizeOfLeaf(leaf));
    }

    void copy(Object[] leaf, int offset, int size) {
      if ( count + size > MAX_TUPLES ) {
        int diff = MAX_TUPLES - size;
        System.arraycopy(leaf, offset, buffer, count, diff);
        offset += diff;
        overflow(leaf[offset++]); // overflow will reset count to 0;
        size -= diff + 1;
      }

      System.arraycopy(leaf, offset, buffer, count, size);
      count += size;
    }

    void prepend(Object[] predecessor, Object predecessorSplit) {

      assert !hasOverflow();

      int pSize = sizeOfLeaf(predecessor);
      int newPos = pSize + 1;
      if ( newPos + count <= MAX_TUPLES ) {
        System.arraycopy(buffer, 0 , buffer, newPos, count); // shift current buffer to get enough room for predecessor tuples.
        System.arraycopy(predecessor, 0, buffer, 0, pSize);
        buffer[pSize] = predecessorSplit;
        count += newPos;
      } else {
        throw new RuntimeException("TODO");
      }
    }
  }

  static class InternalBuilder extends NodeBuilder {

    final LeafBuilder leaf;

    int[] childSizes;
    int[] overflowChildSizes;

    boolean tupleTurn;

    InternalBuilder(NodeBuilder childBuilder) {
      super(childBuilder);
      buffer = new Object[2 * (MAX_TUPLES + 1)];
      childSizes = new int[MAX_TUPLES + 1];
      leaf = childBuilder instanceof LeafBuilder ? (LeafBuilder) childBuilder : ((InternalBuilder) childBuilder).leaf;
    }

    final void init(Object[] node) {

      assert isEmpty();
      origin = node;
      count = tupleSizeOfInternal(node);
      tupleTurn = true;
      System.arraycopy(node, 0, buffer, 0, count);
      System.arraycopy(node, count, buffer, MAX_TUPLES, count+1);
    }

    /**
     * Addition should follow below sequence.
     *  child, tuple, child, tuple, ...., tuple, child.
     */
    final void addTuple(Object tuple) {
  
      assert tupleTurn;
      tupleTurn = false;

      if ( count == MAX_TUPLES ) {
        overflow(tuple);
      } else {
        buffer[count++] = tuple;
      }
    }

    final void addChild(Object[] child, int childSize) {
      assert !tupleTurn;
      assert child != null;
      tupleTurn = true;

      buffer[count + MAX_TUPLES] = child;

      maybeRecordChildSize(childSize);
    }
  
    final void maybeRecordChildSize(int childSize) {
      if ( childSizes != null ) childSizes[count] = childSize;
    }

    final void addChildAndTuple(Object[] child, int childSize, Object tuple) {
      addChild(child, childSize);
      addTuple(tuple);
    }

    final void overflow(Object tuple) {
      throw new RuntimeException("TODO: overflow");
    }

    final Object[] flush() {
      throw new RuntimeException("TODO");
    }
    
    final void flushToParent(InternalBuilder parentBuilder) {
      throw new RuntimeException("TODO");
    }

    final void flushOverflow() {
      setZoomMap(overflowBuffer, MAX_TUPLES);
      parent().addChildAndTuple(overflowBuffer, sizeOfInternal(overflowBuffer), guideTuple);
      guideTuple = null;
      overflowBuffer = null;
    }

    void setZoomMap(Object[] internal, int tupleSize) {
      //IMPROVE: dense case.
      int[] sizes = overflowChildSizes;
      if ( tupleSize < MAX_TUPLES ) {
        sizes = Arrays.copyOf(sizes, tupleSize + 1);
      } else {
        overflowChildSizes = null;
      }
      convSizesToPreSum(sizes);
      internal[2*tupleSize + 1] = new ZoneMap(sizes);
    }

    private static int convSizesToPreSum(int[] sizes, int count) {
      int total = sizes[0];
      for ( int i = 1 ; i < count ; ++i ) {
        sizes[i] = total += 1 + sizes[i];
      }
      return total;
    }

    private static int convSizesToPreSum(int[] sizes) {
      return convSizesToPreSum(sizes, sizes.length);
    }
  }
}