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

    Object[] precedenceBuffer; // cache precede node.
    Object precedenceNext;

    NodeBuilder(NodeBuilder child) {
      this.child = child;
      this.height = child == null ? 1 : 1 + child.height;
    }

    final boolean hasPrecedence() {
      return precedenceNext != null;
    }

    final boolean isSufficient() {
      return count >= MIN_TUPLES || hasPrecedence();
    }

    final boolean mustRebalance() {
      return count < MIN_TUPLES && hasPrecedence();
    }

    final boolean isEmpty() {
      return count == 0 && !hasPrecedence();
    }

    abstract void addTuple(Object tuple);

    abstract Object[] flush();
    abstract void flushToParent(InternalBuilder parentBuilder);

    final Object[] build() {
      NodeBuilder cur = this;

      while ( true ) {
        if ( !cur.hasPrecedence() ) {
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
        batchPrecedence(tuple);
      } else {
        buffer[count++] = tuple;
      }
    }

    void batchPrecedence(Object tuple) {

      if ( hasPrecedence() ) {
        pushPrecedence();
      }

      precedenceNext = tuple;
      precedenceBuffer = buffer;
      buffer = new Object[MAX_TUPLES];
      count = 0;
    }

    void pushPrecedence() {
      parent().addChildAndTuple(precedenceBuffer, MAX_TUPLES, precedenceNext);
      precedenceBuffer = null;
      precedenceNext = null;
    }

    Object[] flush() {

      assert !hasPrecedence();

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

        int steal = MIN_TUPLES - count;
        System.arraycopy(precedenceBuffer, MAX_TUPLES - steal - 1, leaf, 0, steal - 1);
        leaf[steal-1] = precedenceNext;
        System.arraycopy(buffer, 0, leaf, steal, count);

        // Adjust overflow buffer and guide tuple.
        int predecessorRemaining = MAX_TUPLES - steal;
        Object[] predecessor = new Object[predecessorRemaining | 1];
        System.arraycopy(precedenceBuffer, 0, predecessor, 0, predecessorRemaining);
        parent().addChildAndTuple(predecessor, predecessorRemaining, precedenceBuffer[predecessorRemaining]);

        precedenceNext = null;
      } else {
        if ( hasPrecedence() ) {
          pushPrecedence();
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
        batchPrecedence(leaf[offset++]); // overflow will reset count to 0;
        size -= diff + 1;
      }

      System.arraycopy(leaf, offset, buffer, count, size);
      count += size;
    }

    void prepend(Object[] predecessor, Object precedenceNext) {

      assert !hasPrecedence();

      int pSize = sizeOfLeaf(predecessor);
      int newPos = pSize + 1;
      if ( newPos + count <= MAX_TUPLES ) {
        System.arraycopy(buffer, 0 , buffer, newPos, count); // shift current buffer to get enough room for predecessor tuples.
        System.arraycopy(predecessor, 0, buffer, 0, pSize);
        buffer[pSize] = precedenceNext;
        count += newPos;
      } else {
        throw new RuntimeException("TODO");
      }
    }
  }

  static class InternalBuilder extends NodeBuilder {

    final LeafBuilder leaf;

    int[] sizes;
    int[] precedenceSizes;

    boolean hasEndChild;

    InternalBuilder(NodeBuilder childBuilder) {
      super(childBuilder);
      buffer = new Object[2 * (MAX_TUPLES + 1)];
      sizes = new int[MAX_TUPLES + 1];
      leaf = childBuilder instanceof LeafBuilder ? (LeafBuilder) childBuilder : ((InternalBuilder) childBuilder).leaf;
    }

    final void init(Object[] node) {

      assert isEmpty();
      origin = node;
      count = tupleSizeOfInternal(node);
      hasEndChild = true;
      System.arraycopy(node, 0, buffer, 0, count);
      System.arraycopy(node, count, buffer, MAX_TUPLES, count+1);
    }

    /**
     * Addition should follow below sequence.
     *  child, tuple, child, tuple, ...., tuple, child.
     */
    final void addTuple(Object tuple) {
  
      assert hasEndChild;
      hasEndChild = false;

      if ( count == MAX_TUPLES ) {
        batchPrecedence(tuple);
      } else {
        buffer[count++] = tuple;
      }

    }

    final void addChild(Object[] child, int childSize) {
      assert !hasEndChild;
      assert child != null;
      hasEndChild = true;


      buffer[count + MAX_TUPLES] = child;

      maybeRecordChildSize(childSize);
    }

    final void addChildAndTuple(Object[] child, int childSize, Object tuple) {
      addChild(child, childSize);
      addTuple(tuple);
    }
  
    final void maybeRecordChildSize(int childSize) {
      if ( sizes != null ) sizes[count] = childSize;
    }

    final void batchPrecedence(Object tuple) {
      assert hasEndChild;

      if ( hasPrecedence() ) {
        pushPrecedence();
      }

      precedenceBuffer = buffer;
      precedenceSizes = sizes;
      precedenceNext = tuple;

      count = 0;
      buffer = new Object[2*(MAX_TUPLES + 1)];
      sizes = new int[MAX_TUPLES+1];
    }

    final void pushPrecedence() {
      setZoomMap(precedenceBuffer, MAX_TUPLES);
      parent().addChildAndTuple(precedenceBuffer, sizeOfInternal(precedenceBuffer), precedenceNext);
      precedenceNext = null;
      precedenceBuffer = null;
    }

    final Object[] flush() {
      //IMPROVE: use origin if no change insteand of create new object.
      assert hasEndChild;
      assert !hasPrecedence();

      if ( count == 0 ) {
        // return first child.
        hasEndChild = false;
        return (Object[]) buffer[MAX_TUPLES];
      }

      Object[] internal = new Object[2 * (count + 1)];
      if ( count == MAX_TUPLES ) {
        // Skip copy, improve performance.
        Object[] t = buffer;
        buffer = internal;
        internal = t;
      } else {
        System.arraycopy(buffer, 0, internal, 0, count);
        System.arraycopy(buffer, MAX_TUPLES, internal, count, count+1);
      }

      setZoomMap(internal, count, sizes);

      count = 0;
      hasEndChild = false;
      return internal;
    }
    
    final void flushToParent(InternalBuilder parentBuilder) {
      assert hasEndChild;

      int internalSize;
      Object[] internal;

      if ( mustRebalance() ) {
        int steal = MIN_TUPLES - count;
        internal = new Object[2 * (MIN_TUPLES + 1)];
        // steal precedence tuples
        System.arraycopy(precedenceBuffer, MAX_TUPLES - (steal - 1), internal, 0, steal - 1);
        internal[steal - 1] = precedenceNext;
        System.arraycopy(buffer, 0, internal, steal, count);
        // steal precedence children
        System.arraycopy(precedenceBuffer, 2 * MAX_TUPLES + 1 - steal, internal, MIN_TUPLES, steal);
        System.arraycopy(buffer, MAX_TUPLES, internal, MIN_TUPLES + steal, count + 1);

        // Rebalance & create zoomap.
        int[] childSizes = new int[MIN_TUPLES + 1];
        System.arraycopy(precedenceSizes, MAX_TUPLES + 1 - steal, childSizes, 0, steal);
        System.arraycopy(sizes, 0, childSizes, steal, count + 1);
        setZoomMap(internal, MIN_TUPLES, childSizes);

        throw new RuntimeException("TODO");
      } else {

        if ( hasPrecedence() ) {
          pushPrecedence();
        }

        assert count > 0;
        internal = new Object[2 * (count + 1)];
        System.arraycopy(buffer, 0, internal, 0, count);
        System.arraycopy(buffer, MAX_TUPLES, internal, count, count+1);
        setZoomMap(internal, count, sizes);
        internalSize = sizeOfInternal(internal);
      }
    
      count = 0;
      hasEndChild = false;
      if ( parentBuilder != null ) {
        parentBuilder.addChild(internal, internalSize);
      }
    }

    void setZoomMap(Object[] internal, int tupleSize, int[] sizes) {
      var childSize = tupleSize + 1;
      var preSum = Arrays.copyOf(sizes, childSize);
      convSizesToPreSum(preSum);

      internal[2*tupleSize+1] = new ZoneMap(preSum);
    }

    void setZoomMap(Object[] internal, int tupleSize) {
      //IMPROVE: dense case.
      int[] sizes = precedenceSizes;
      if ( tupleSize < MAX_TUPLES ) {
        sizes = Arrays.copyOf(sizes, tupleSize + 1);
      } else {
        //Imply: tupleSize == MAX_TUPLES
        precedenceSizes = null;
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