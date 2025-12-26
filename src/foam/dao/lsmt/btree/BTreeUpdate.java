/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.Arrays;
import java.util.Comparator;
import java.util.function.BiFunction;
import static foam.dao.lsmt.btree.BTree.*;

public class BTreeUpdate {

  public static interface UpdateFunction<O, N> {
    O insert(N insert);
    O merge(O exist, N insert);
  }

  /**
   * Insert tuple class is same as tuples in the tree.
   */
  public static class SimpleUpdate<N> implements UpdateFunction<N, N> {

    private final BiFunction<N,N,N> wrapped;
    public SimpleUpdate(BiFunction<N,N,N> wrapped) {
      this.wrapped = wrapped;
    }

    @Override
    public N insert(N insert) {
      return insert;
    }

    @Override
    public N merge(N exist, N insert) {
      return wrapped.apply(exist, insert);
    }

    public static <N> SimpleUpdate<N> of(BiFunction<N,N,N> bf) {
      return new SimpleUpdate<N>(bf);
    }
  }

  // Insert only, update will keep original tuple.
  static final SimpleUpdate<Object> NO_OP = SimpleUpdate.of((o, n) -> n);

  static boolean isSimple(UpdateFunction<?,?> updater) {
    return updater instanceof SimpleUpdate;
  }

  public static <C, O extends C, N extends C> Object[] merge(
    Comparator<? super C> comparator, UpdateFunction<O, N> updater, Object[] oNode, Object[] nNode
  ) {
    if ( isEmpty(nNode) ) return oNode;

    if ( isEmpty(oNode) ) {
      if ( isSimple(updater) ) {
        // if ( updater == NO_OP ) {
        //   return nNode;
        // } else {
        //   throw new RuntimeException("TODO: transfer function");
        // }
        return nNode;
      }
    }

    if ( isLeaf(oNode) && isLeaf(nNode) ) {
      if ( updater == NO_OP && oNode.length < nNode.length ) {
        Object[] tmp = oNode;
        oNode = nNode;
        nNode = tmp;
      }

      return mergeLeaves(comparator, updater, oNode, nNode);
    }

    //IMPROVE: nNode is not leaf

    try ( TreeInternalBuilder builder = new TreeInternalBuilder() ) {
      return builder.update(comparator, updater, oNode, nNode);
    }
  }

  public static <C, O extends C, N extends C> Object[] mergeLeaves(
    Comparator<? super C> comparator, UpdateFunction<O, N> updater, Object[] oNode, Object[] nNode
  ) {

    int oPos = -1, oSize = sizeOfLeaf(oNode);
    O oPeek = (O) oNode[0];

    int nPos = 0, nSize = sizeOfLeaf(nNode);
    N nPeek = (N) nNode[0];

    /**
     * Optimisation: find postion in the oNode in which preceding tuples can be copy to the result tree directly.
     * break at position where oPeek > nPeek or oPeek != nPeek.
     */
    O merged = null;
    int compRes = -1; // compare(old, new)
    while ( compRes <= 0 ) {
      if ( compRes < 0 ) {

        oPos = search(comparator, oNode, oPos + 1, oSize, nPeek);
        compRes = oPos < 0 ? 1 : 0;
        if ( oPos < 0 ) {
          oPos = -(1 + oPos);
        }
        // IMPLY: tuples GTE nPos in nNode is greater than tuples in oNode.
        if ( oPos == oSize ) {
          break;
        }
        oPeek = (O) oNode[oPos];
      } else {
        //IMPLY: compRes == 0
        merged = updater.merge(oPeek, nPeek);
        if ( merged != oPeek) break;
        // if new node is exactly equal to old node, then return old node.
        if ( ++nPos == nSize ) return oNode;
        // IMPLY: old node is a prefix of new node.
        if ( ++oPos == oSize ) break; 
        
        compRes = comparator.compare(oPeek = (O) oNode[oPos], nPeek = (N) nNode[nPos]);
      }
    }


    try ( TreeLeafBuilder<O> builder = createBuilder() ) {

      /**
       * upos > 0 => [0, oPos-1] in the oNode can be directly copied to result, 
       *              as it is less than the smallest tuple in the nNode.
       */
      if ( oPos > 0 ) {
        builder.leaf().copy(oNode, 0, oPos);
      }

      if ( oPos < oSize ) {

        /**
         * compRes == 0 => a old tuple is updated with new one.
         * compRes > 0 => the next tuple is nPeek.
         */
        if ( compRes == 0 ) {
          builder.add(merged);
          if ( ++oPos < oSize ) {
            oPeek = (O) oNode[oPos];
          }
        } else {
          builder.add(updater.insert(nPeek));
        }

        if ( ++nPos < nSize ) nPeek = (N) nNode[nPos];

        if ( oPos < oSize && nPos < nSize ) {

          compRes = comparator.compare(oPeek, nPeek);

          while ( true ) {

            if ( compRes == 0 ) {

              builder.leaf().addTuple(updater.merge(oPeek, nPeek));
              ++oPos;
              ++nPos;
              if ( oPos == oSize || nPos == nSize ) break;
              compRes = comparator.compare(oPeek = (O) oNode[oPos], nPeek = (N) nNode[nPos]);

            } else if ( compRes < 0 ) {

              int jump = search(comparator, oNode, oPos + 1, oSize, nPeek);
              compRes = jump < 0 ? 1 : 0; // jump < 0 => find position in oNode that is greater than nPeak.
              if ( jump < 0 ) {
                jump = -(jump + 1);
              }
              builder.leaf().copy(oNode, oPos, jump - oPos);
              if ( (oPos = jump) == oSize ) break;
              oPeek = (O) oNode[oPos];

            } else {

              int jump = search(comparator, nNode, nPos + 1, nSize, oPeek);
              compRes = jump < 0 ? -1 : 0; // jump < 0 => find position in nNode that is greater than oPeek.
              if ( jump < 0 ) {
                jump = -(jump + 1);
              }
              builder.leaf().copy(nNode, nPos, jump - nPos, updater);
              if ( (nPos = jump) == nSize ) break;
              nPeek = (N) nNode[nPos];

            }
          }
        }

        if ( oPos < oSize ) {
          //IMPLY: all tuples in the nNode are inserted in the tree.
          builder.leaf().copy(oNode, oPos, oSize - oPos);
        }
      }


      if ( nPos < nSize ) {
        builder.leaf().copy(nNode, nPos, nSize - nPos, updater);
      }
      return builder.build();
    }
  }

  static <T> TreeLeafBuilder<T> createBuilder() {
    return new TreeLeafBuilder();
  }

  private static abstract class AbstractTreeLeafBuilder extends LeafBuilder {

    final LeafBuilder leaf() {
      return this;
    }

    abstract void reset();

  }

  public static class TreeLeafBuilder<T> extends AbstractTreeLeafBuilder implements AutoCloseable {

    final public void add(Object[] src , int offset, int size) {
      leaf().copy(src, offset, size);
    }

    final public void add(T tuple) {
      leaf().addTuple(tuple);
    }

    @Override
    final public void close() {
      //TODO: threadlocal pool(Object poll).
      // uncommon below method when reusing builder is done.
      // reset();
    }

    @Override
    void reset() {
      Arrays.fill(leaf().buffer, null);
      leaf().count = 0;
      InternalBuilder internal = leaf().parent;
      while ( internal != null ) {
        internal.reset();
        internal = internal.parent;
      }
    }
  }

  private static abstract class AbstractTreeInternalBuilder extends AbstractTreeLeafBuilder {

    @Override
    void reset() {
      throw new RuntimeException("TODO: AbstractTreeInternalBuilder.reset method");
    }

  }

  static class TreeInternalBuilder<C, O extends C, N extends C> extends AbstractTreeInternalBuilder implements AutoCloseable {

    final TreeIterator<C, N> nIterator = new TreeIterator<>();
    Comparator<? super C> comparator;
    UpdateFunction<O,N> updater;
    
    Object[] update(Comparator<? super C> comparator, UpdateFunction<O, N> updater, Object[] oNode, Object[] nNode) {

      this.nIterator.init(nNode);
      this.updater = updater;
      this.comparator = comparator;
      NodeBuilder builder = leaf();

      assert builder.isEmpty();

      for ( int i = 0 ; i < BTree.height(oNode) - 1 ; i++ ) {
        InternalBuilder internal = builder.parent();
        assert internal.isEmpty() && !internal.hasEndChild;
        builder = internal;
      }

      N nPeek = this.nIterator.next();
      nPeek = merge(nPeek, oNode, null, builder);
      assert nPeek == null;
      Object[] result = builder.build();

      return result;
    }

    private N merge(N nPeek, Object[] oNode, O oBound, NodeBuilder builder) {
      return builder == leaf() ?
        mergeLeaf(nPeek, oNode, oBound, (LeafBuilder) builder) :
        mergeInternal(nPeek, oNode, oBound, (InternalBuilder) builder);
    }

    private N mergeInternal(N nPeek, Object[] oNode, O oBound, InternalBuilder builder) {
      int oPos = 0;
      int oSize = tupleSizeOfInternal(oNode);

      while ( nPeek != null ) {
        
        int oJump = searchWithUpperBound(comparator, oNode, oPos, oSize, oBound, nPeek);
        int c = oJump >= 0 ? 0 : -1;
        if ( oJump < 0 ) {
          oJump = -(1 + oJump);
        }

        if ( oJump > oSize ) break; // nPeak is greater than all tuples in oNode.

        if ( oJump > oPos ) {
          builder.copyPre(oNode, oSize, oPos, oJump - oPos);
        }

        //IMPLY: oJump must be <= oSize.
        O nextBound = oJump < oSize ? (O) oNode[oJump] : oBound;
        Object[] child = (Object[]) oNode[oJump + oSize];

        // c is less or equal to 0.

        if ( c < 0 ) {
          //IMPLY: the insert position of nPeek is inside the child.
          NodeBuilder childBuilder = builder.child;
          //TODO: use origin? how it works?
          
          nPeek = merge(nPeek, child, nextBound, childBuilder);
          childBuilder.flushToParent(builder);
          if ( oJump == oSize ) {
            //IMPLY: child is the most left child.
            return nPeek;
          }
          // nNode is exhausted, so set nPeek to infinite.
          c = nPeek == null ? -1 : comparator.compare(nextBound, nPeek);
          builder.addTuple(nextBound);
          
        } else {
          // c == 0 => tuples in the child are less than nPeek
          builder.addChild(child, size(child));
          builder.addTuple(updater.merge(nextBound, nPeek));
          nPeek = nIterator.next();
        }

        oPos = oJump + 1;
      }

      if ( oPos <= oSize ) {
        builder.copyPre(oNode, oSize, oPos, oSize - oPos);
        Object[] lastChild = (Object[]) oNode[oSize * 2];
        builder.addChild(lastChild, size(lastChild));
      }
      return nPeek;
    }

    private N mergeLeaf(N nPeek, Object[] oNode, O oBound, LeafBuilder builder) {

      int oPos = 0;
      int oSize = sizeOfLeaf(oNode);
      O oPeek = (O) oNode[oPos];
      int c = comparator.compare(oPeek, nPeek);

      /**
       * c == 0 => oPeek == nPeek
       * c < 0  => oPeek < nPeek
       * c > 0  => oPeek > nPeek
       */
      while ( true ) {
        if ( c == 0 ) {
          leaf().addTuple(updater.merge(oPeek, nPeek));
          if ( ++oPos < oSize ) {
            oPeek = (O) oNode[oPos];
          }
          nPeek = nIterator.next();
          if ( nPeek == null ) { // nIterater exhasuted.
            builder.copy(oNode, oPos, oSize - oPos);
            return null;
          }
          if ( oPos == oSize ) break;

          c = comparator.compare(oPeek, nPeek);
        } else if ( c < 0 ) {
          // find offiset in oNode that is also less than nPeek.
          int oJump = search(comparator, oNode, oPos + 1, oSize, nPeek);
          c = oJump >= 0 ? 0 : 1;
          if ( oJump < 0 ) {
            oJump = -(1 + oJump);
          }
          builder.copy(oNode, oPos, oJump - oPos);
          if ( (oPos = oJump) == oSize ) {
            break;
          }
          oPeek = (O) oNode[oPos];
        } else {
          // int copyKeysLT(C upperBound, Comparator<? super C> comparator, LeafBuilder builder, UpdateFunction<O, N> updater)
          builder.addTuple(isSimple(updater) ? nPeek : updater.insert(nPeek));
          c = nIterator.copyKeysLT(oPeek, comparator, builder, updater);
          nPeek = nIterator.next();
          if ( nPeek == null ) {
            //IMPLY: nNode is exhausted.
            builder.copy(oNode, oPos, oSize - oPos);
            return null;
          }
        }
      }

      /**
       * oBound == null => oBound == positive infinite.
       */

      if ( oBound == null || comparator.compare(nPeek, oBound) < 0 ) {
        builder.addTuple(isSimple(updater) ? nPeek : updater.insert(nPeek));
        nIterator.copyKeysLT(oBound, comparator, builder, updater);
        nPeek = nIterator.next();
      }

      return nPeek;
    }

    @Override
    public void close() {

    }
  }

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
      return false;
    }

    static boolean areChildSizesIdentical(int[] a, int aOffset, int[] b, int bOffset, int count) {
      for ( int i = 0 ; i < count ; i++ ) {
        if ( a[i+aOffset] != b[i+bOffset] ) return false;
      }
      return true;
    }
  }

  static abstract class LeafBuilder extends NodeBuilder {

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
        // steal tuples from precedence buffer to make last node reach to MIN_TUPLES.
        leafSize = MIN_TUPLES;
        leaf = new Object[MIN_TUPLES];

        int diff = MIN_TUPLES - count;
        System.arraycopy(precedenceBuffer, MAX_TUPLES - diff + 1, leaf, 0, diff - 1); // steal one less for precedenceNext.
        leaf[diff-1] = precedenceNext;
        System.arraycopy(buffer, 0, leaf, diff, count);

        // Adjust precedence buffer and guide tuple.
        int predecessorRemaining = MAX_TUPLES - diff;
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

    void copy(Object[] src, int offset, int size) {
      if ( count + size > MAX_TUPLES ) {
        int diff = MAX_TUPLES - count;
        // System.out.println("AAA: " + src.length + " , offset: " + offset + ", size: " + size + ", diff: " + diff + ", count: " + count);
        System.arraycopy(src, offset, buffer, count, diff);
        offset += diff;
        batchPrecedence(src[offset++]);
        size -= diff + 1;
      }

      System.arraycopy(src, offset, buffer, count, size);
      count += size;
    }

    <O, N> void copy(Object[] src, int offset, int size, UpdateFunction<O, N> updater) {
      
      if ( isSimple(updater) ) {
        copy(src, offset, size);
        return;
      }

      if ( count + size > MAX_TUPLES ) {
        int diff = MAX_TUPLES - count;
        for ( int i = 0 ; i < diff ; i++ ) {
          buffer[count + i] = updater.insert((N) src[offset + i]);
        }
        offset += diff;
        batchPrecedence(updater.insert((N) src[offset++]));
        size -= diff + 1;
      }

      for ( int i = 0 ; i < size ; i++ ) {
        buffer[count + i] = updater.insert((N) src[offset + i]);
      }

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
      //CHECK: sizeMap.
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
      assert !hasEndChild;

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
      copyPrecedencePresum(precedenceBuffer, MAX_TUPLES);
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

        applyBufferPresum(internal, count);
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
        int diff = MIN_TUPLES - count;
        internal = new Object[2 * (MIN_TUPLES + 1)];
        // diff precedence tuples
        System.arraycopy(precedenceBuffer, MAX_TUPLES - (diff - 1), internal, 0, diff - 1);
        internal[diff - 1] = precedenceNext;
        System.arraycopy(buffer, 0, internal, diff, count);
        // diff precedence children
        System.arraycopy(precedenceBuffer, 2 * MAX_TUPLES + 1 - diff, internal, MIN_TUPLES, diff);
        System.arraycopy(buffer, MAX_TUPLES, internal, MIN_TUPLES + diff, count + 1);

        // Rebalance & create zoomap.
        int[] presum = new int[MIN_TUPLES + 1];
        System.arraycopy(precedenceSizes, MAX_TUPLES + 1 - diff, presum, 0, diff);
        System.arraycopy(sizes, 0, presum, diff, count + 1);
        internalSize = convSizesToPresum(presum, MIN_TUPLES + 1);
        internal[2*MIN_TUPLES + 1] = presum;

        // refactor&push precedence to parent.
        int remainingTuples = MAX_TUPLES - diff;
        Object[] preInternal = new Object[2 * (remainingTuples + 1)];
        System.arraycopy(precedenceBuffer, 0, preInternal, 0, remainingTuples);
        System.arraycopy(precedenceBuffer, MAX_TUPLES, preInternal, remainingTuples, remainingTuples+1);
        copyPrecedencePresum(preInternal, remainingTuples);
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
        applyBufferPresum(internal, count);
        internalSize = sizeOfInternal(internal);
      }
    
      count = 0;
      hasEndChild = false;
      if ( parentBuilder != null ) {
        parentBuilder.addChild(internal, internalSize);
      }
    }

    void applyBufferPresum(Object[] toInternal, int tupleSize) {
      
      int[] presum = this.sizes;

      if ( tupleSize < MAX_TUPLES ) {
        presum = Arrays.copyOf(presum, tupleSize+1);
      } else {
        this.sizes = new int[MAX_TUPLES+1];
      }
      convSizesToPresum(presum, tupleSize + 1);
      toInternal[2*tupleSize + 1] = presum;
    }

    void copyPrecedencePresum(Object[] internal, int tupleSize) {
      //IMPROVE: shotcut for full node.

      int[] presum = precedenceSizes;
      if ( tupleSize < MAX_TUPLES ) {
        presum = Arrays.copyOf(presum, tupleSize+1);
      } else {
        precedenceSizes = null; // batchProcedence method will initial a new one.
      }

      convSizesToPresum(presum, tupleSize+1);
      internal[2 * tupleSize + 1] = presum;
    }

    static int convSizesToPresum(int[] sizeMap, int numOfChild) {
      int total = sizeMap[0];
      for ( int i = 1 ; i < numOfChild ; ++i ) {
        sizeMap[i] = total += 1 + sizeMap[i];
      }
      return total;
    }

    void prepend(Object[] pred, Object predNext) {
      assert !hasPrecedence();

      int preTupleSize = tupleSizeOfInternal(pred);
      int[] presum = getPresum(pred);
      int offOrCSize = 1 + preTupleSize; // offset or childSize, both are equal.

      if ( offOrCSize + count <= MAX_TUPLES ) {
        //IMPL: buffer has enough spaces to store pred without the need from precedenceBuffer.

        // right shift tuples to make room.
        System.arraycopy(buffer, 0, buffer, offOrCSize, count);
        System.arraycopy(buffer, 0, buffer, MAX_TUPLES + offOrCSize, count+1);
        System.arraycopy(sizes, 0, sizes, offOrCSize, count+1); 

        // insert pred tuples.
        System.arraycopy(pred, 0, buffer, 0, preTupleSize);
        System.arraycopy(pred, preTupleSize, buffer, MAX_TUPLES, offOrCSize);
        buffer[preTupleSize] = predNext;
        convPresumToSizes(presum, 0, sizes, 0, offOrCSize);

        count += offOrCSize;

      } else {
        
        if ( precedenceBuffer == null ) {
          precedenceBuffer = new Object[2 * (MAX_TUPLES + 1)];
          precedenceSizes = new int[MAX_TUPLES + 1];
        }

        System.arraycopy(pred, 0, precedenceBuffer, 0, preTupleSize);
        System.arraycopy(pred, preTupleSize, precedenceBuffer, MAX_TUPLES, preTupleSize+1);
        convPresumToSizes(presum, 0, precedenceSizes, 0, preTupleSize+1);

        if ( preTupleSize == MAX_TUPLES ) {
          precedenceNext = predNext;
        } else {
          // move tuples from buffer to precedence to make precedence a FULL node.
          
          int miss = MAX_TUPLES - preTupleSize;

          precedenceBuffer[preTupleSize] = predNext;
          System.arraycopy(buffer, 0, precedenceBuffer, preTupleSize+1, miss-1);
          System.arraycopy(buffer, MAX_TUPLES, precedenceBuffer, MAX_TUPLES + preTupleSize +1 , miss);
          System.arraycopy(sizes, 0, precedenceSizes, preTupleSize+1, miss); 
          precedenceNext = buffer[miss-1];

          System.arraycopy(buffer, miss, buffer, 0, count-miss);
          System.arraycopy(buffer, MAX_TUPLES + miss, buffer, MAX_TUPLES, count-miss+1);
          System.arraycopy(sizes, miss, sizes, 0, count-miss+1);
          count = count-miss;
        }
      }
    }
  
    void copyPre(Object[] copy, int tupleSize, int offset, int size) {
  
      assert !hasEndChild;

      int[] presum = getPresum(copy);
      if ( count + size > MAX_TUPLES ) {
      
        int diff = MAX_TUPLES - count;
        copyPreNoBatch(copy, tupleSize, presum,  offset, diff);
        offset += diff;

        // copy END child.
        buffer[2*MAX_TUPLES] = copy[tupleSize + offset];
        sizes[MAX_TUPLES] = presum[offset] - (offset > 0 ? (presum[offset-1] + 1) : 0);

        batchPrecedence(copy[offset]);

        size -= diff + 1;
        ++offset;
      }

      copyPreNoBatch(copy, tupleSize, presum, offset, size);
    }
  
    private void copyPreNoBatch(Object[] copy, int tupleSize, int[] presum, int offset, int length) {
      assert !hasEndChild;

      if ( length == 0 ) return;

      if ( length == 1 ) {
        buffer[count] = copy[offset];
        buffer[MAX_TUPLES + count] = copy[tupleSize + offset];
        sizes[count] = presum[offset] - (offset > 0 ? (presum[offset-1] + 1) : 0);
        ++count;
      } else {
        System.arraycopy(copy, offset, buffer, count, length);
        System.arraycopy(copy, tupleSize + offset, buffer, MAX_TUPLES + count, length);
        convPresumToSizes(presum, offset, sizes, count, length);
        count += length;
      }

    }

    static void convPresumToSizes(int[] in, int inOffset, int[] out, int outOffset, int length) {
      assert length > 0;

      //TODO:
      if ( inOffset == 0 ) {
        out[outOffset++] = in[inOffset++];
        --length;
      }

      for ( int i = 0 ; i < length ; ++i ) {
        out[outOffset + i] = in[inOffset + i] - (in[inOffset + i - 1] + 1);
      }
    }
  
    void reset() {
      Arrays.fill(buffer, null);
      count = 0;
      hasEndChild = false;
      clearOrigin();
    }
    // void prepend()
  }

  private static class TreeIterator<C, N extends C> {

    /**
     * depth, levels and position do not count for leaf.
     */
    Object[][] levels;
    int[] positions; // record index of child it is currectly in in the internal node.
    int depth;

    Object[] leaf;
    int leafSize, leafPos;

    void init(Object[] tree) {
      int minHeight = requireHeight(size(tree));
      if ( positions == null || minHeight > positions.length ) {
        positions = new int[minHeight];
        levels = new Object[minHeight][];
      }
      depth = 0;
      downToLeftMostLeaf(tree);
    }

    N next() {
      if ( leafPos < leafSize ) return (N) leaf[leafPos++];

      // complete all node.
      if ( depth == 0 ) return null;

      Object[] node = levels[depth-1];
      int position = positions[depth-1];
      N result = (N) node[position];
      forwardToNextLeaf(node, position + 1);
      return result;
    }

    private void forwardToNextLeaf(Object[] node, int position) {
      int tupleSize = firstChildOfInternal(node);
      if ( position < tupleSize ) {
        positions[depth-1] = position;
      } else {
        --depth; // only last child left
      }
      downToLeftMostLeaf((Object[]) node[tupleSize + position]);
    }

    private void downToLeftMostLeaf(Object[] node) {
      while ( ! isLeaf(node) ) {
        levels[depth] = node;
        positions[depth] = 0;
        node = (Object[]) node[firstChildOfInternal(node)];
        ++depth;
      }
      leaf = node;
      leafPos = 0;
      leafSize = sizeOfLeaf(node);
    }

    void reset() {
      leaf = null;
      Arrays.fill(levels, 0, levels.length, null);
    }

    /**
     * add all tuples in the iterator that are less than upperBound.
     */
    <O> int copyKeysLT(C upperBound, Comparator<? super C> comparator, LeafBuilder builder, UpdateFunction<O, N> updater) {
      while ( true ) {
        int c = searchWithMaybePosiInfi(comparator, leaf, leafPos, leafSize, upperBound);
        int end = c >= 0 ? c : -(c + 1);

        if ( end > leafPos ) {
          builder.copy(leaf, leafPos, end - leafPos, updater);
          leafPos = end;
        }

        if ( end < leafSize ) {
          // IMPLY: upperBound hit.
          // 0: match found, otherwise: -1
          return c >> 31;
        }

        // complete all node.
        if ( depth == 0 ) return -1;

        // IMPLY: upperBound not hit in current leaf, we can move forward to next leaf.

        Object[] node = levels[depth-1];
        int position = positions[depth-1];
        N internalTuple = (N) node[position];
        c = compareWithMaybePosiInfi(comparator, internalTuple, upperBound);
        if ( c >= 0 ) return -c;

        builder.addTuple(isSimple(updater) ? internalTuple : updater.insert(internalTuple));
        forwardToNextLeaf(node, position - 1);
      }
    }
  }
}