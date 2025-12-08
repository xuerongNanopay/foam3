/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.Arrays;
import static foam.dao.lsmt.btree.BTree.*;

public class BTreeUpdate {

  /**
   * Reusable builder.
   * flush and flushParent will reset the builder, so builder can be reuse again.
   */
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

    final void setOrigin(Object[] origin) {
      this.origin = origin;
    }

    final void clearOrigin() {
      this.origin = null;
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

    boolean produceFullNode() {
      return true;
    }

    static boolean areChildSizesIdentical(int[] a, int aOffset, int[] b, int bOffset, int count) {
      for ( int i = 0 ; i < count ; i++ ) {
        if ( a[i+aOffset] != b[i+bOffset] ) return false;
      }
      return true;
    }
  }

  static class LeafBuilder extends NodeBuilder {

    LeafBuilder() {
      super(null);
      this.buffer = new Object[MAX_TUPLES];
    }

    void initial(Object[] origin) {
      assert isEmpty();
      setOrigin(origin);
      count = sizeOfLeaf(origin);
      System.arraycopy(origin, 0, buffer, 0, count);
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

      // precedenceBuffer may not be null after previous build, see rebalance.
      // In this case, we can resuse the buffer instead of creating new Object.
      var newBuf = precedenceBuffer != null ? precedenceBuffer : new Object[MAX_TUPLES];

      precedenceNext = tuple;
      precedenceBuffer = buffer;
      buffer = newBuf;
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
      
      } else if ( 
        ! hasPrecedence() &&
        origin != null &&
        count == sizeOfLeaf(origin) &&
        areNodeIdentical(buffer, 0 , origin, 0, count)
      ) {
        // no change on the origin node so just reuse it.
        leafSize = count;
        leaf = origin;
      } else {
        if ( hasPrecedence() ) {
          pushPrecedence();
        }

        leafSize = count;
        leaf = flush();
      }
      count = 0;
      clearOrigin();
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
        batchPrecedence(leaf[offset++]);
        size -= diff + 1;
      }

      System.arraycopy(leaf, offset, buffer, count, size);
      count += size;
    }

    void prepend(Object[] pred, Object preNext) {

      assert !hasPrecedence();

      int pSize = sizeOfLeaf(pred);
      int newPos = pSize + 1;
      if ( newPos + count <= MAX_TUPLES ) {
        System.arraycopy(buffer, 0 , buffer, newPos, count); // shift current buffer to get enough room for pred tuples.
        System.arraycopy(pred, 0, buffer, 0, pSize);
        buffer[pSize] = preNext;
        count += newPos;
      } else {
        //IMPLY: buffer has enough tuple to make a FULL precedence.
        if ( precedenceBuffer == null ) {
          precedenceBuffer = new Object[MAX_TUPLES];
        }
        System.arraycopy(pred, 0, precedenceBuffer, 0, pSize);

        if ( pSize == MAX_TUPLES ) {
          precedenceNext = preNext;
        } else {
          // move tuples from buffer to precedenceBuffer to make precedence a FULL node.
          int steal = MAX_TUPLES - pSize;
          count -= steal;
          precedenceBuffer[pSize] = preNext;
          System.arraycopy(buffer, 0, precedenceBuffer, pSize + 1, MAX_TUPLES - newPos);
          precedenceNext = buffer[MAX_TUPLES - newPos];
          System.arraycopy(buffer, steal, buffer, 0, count);
        }
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

    final void initial(Object[] node) {

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

      Object[] newBuffer = precedenceBuffer;
      int[] newSizes = precedenceSizes;
    
      precedenceBuffer = buffer;
      precedenceSizes = sizes;
      precedenceNext = tuple;

      buffer = newBuffer == null ? new Object[2*(MAX_TUPLES + 1)] : newBuffer;
      sizes = newSizes == null ? new int[MAX_TUPLES + 1] : newSizes;
      count = 0;
    }

    final void pushPrecedence() {
      setZoneMap(precedenceBuffer, MAX_TUPLES);
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
        clearOrigin();
        hasEndChild = false;
        return (Object[]) buffer[MAX_TUPLES];
      }

      Object[] internal;

      if (
        origin != null &&
        count == tupleSizeOfInternal(origin) &&
        BTree.areShadowIdentical(buffer, 0, origin, 0, count) &&
        BTree.areShadowIdentical(buffer, MAX_TUPLES, origin, count, count + 1)
      ) {
        internal = origin;
      } else {

        internal = new Object[2 * (count + 1)];
        if ( count == MAX_TUPLES ) {
          // Skip copy, improve performance.
          Object[] t = buffer;
          buffer = internal;
          internal = t;
        } else {
          System.arraycopy(buffer, 0, internal, 0, count);
          System.arraycopy(buffer, MAX_TUPLES, internal, count, count+1);
        }

        applyZoneMapFromBuffer(internal, count);
      }

      count = 0;
      hasEndChild = false;
      clearOrigin();
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
        setZoneMap(internal, MIN_TUPLES, childSizes);
        internalSize = sizeOfInternal(internal);

        // refactor&push precedence to parent.
        int remainingTuples = MAX_TUPLES - steal;
        Object[] preInternal = new Object[2 * (remainingTuples + 1)];
        System.arraycopy(precedenceBuffer, 0, preInternal, 0, remainingTuples);
        System.arraycopy(precedenceBuffer, MAX_TUPLES, preInternal, remainingTuples, remainingTuples+1);
        applyPrecedenceZoneMap(preInternal, remainingTuples);
        parent().addChildAndTuple(preInternal, sizeOfInternal(preInternal), precedenceBuffer[remainingTuples]);
        precedenceNext = null;
      } else {

        if ( hasPrecedence() ) {
          pushPrecedence();
        }

        assert count > 0;
        internal = new Object[2 * (count + 1)];
        System.arraycopy(buffer, 0, internal, 0, count);
        System.arraycopy(buffer, MAX_TUPLES, internal, count, count+1);
        setZoneMap(internal, count, sizes);
        internalSize = sizeOfInternal(internal);
      }
    
      count = 0;
      hasEndChild = false;
      if ( parentBuilder != null ) {
        parentBuilder.addChild(internal, internalSize);
      }
    }

    void applyZoneMapFromBuffer(Object[] toInternal, int tupleSize) {
      //IMPROVE: FULL NODE short cut.
      
      int[] preSum = this.sizes;

      if ( tupleSize < MAX_TUPLES ) {
        preSum = Arrays.copyOf(preSum, tupleSize+1);
      } else {
        this.sizes = new int[MAX_TUPLES+1];
      }
      convSizesToPreSum(preSum, tupleSize + 1);
      toInternal[2*tupleSize + 1] = preSum;

    }

    void applyPrecedenceZoneMap(Object[] internal, int tupleSize) {
      //IMPROVE: shotcut for full node.

      int[] childSizes = precedenceSizes;
      if ( tupleSize < MAX_TUPLES ) {
        sizes = Arrays.copyOf(sizes, tupleSize+1);
      } else {
        precedenceSizes = null;
      }
      convSizesToPreSum(childSizes, tupleSize);
      internal[2 * tupleSize + 1] = childSizes;
    }

    void setZoneMap(Object[] internal, int tupleSize, int[] sizes) {
      var childSize = tupleSize + 1;
      var preSum = Arrays.copyOf(sizes, childSize);
      convSizesToPreSum(preSum, tupleSize+1);

      internal[2*tupleSize+1] = new ZoneMap(preSum);
    }

    void setZoneMap(Object[] internal, int tupleSize) {
      //IMPROVE: dense case.
      int[] sizes = precedenceSizes;
      if ( tupleSize < MAX_TUPLES ) {
        sizes = Arrays.copyOf(sizes, tupleSize + 1);
      } else {
        //Imply: tupleSize == MAX_TUPLES
        precedenceSizes = null;
      }
      convSizesToPreSum(sizes, tupleSize+1);
      internal[2*tupleSize + 1] = new ZoneMap(sizes);
    }

    private static int convSizesToPreSum(int[] sizeMap, int numOfChild) {
      int total = sizeMap[0];
      for ( int i = 1 ; i < numOfChild ; ++i ) {
        sizeMap[i] = total += 1 + sizeMap[i];
      }
      return total;
    }

    // private static int convSizesToPreSum(int[] sizes) {
    //   return convSizesToPreSum(sizes, sizes.length);
    // }
  }
}