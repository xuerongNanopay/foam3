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

public class BTreeLeafCursor<K, T> {
  
  private final Comparator<? super K> comparator_;
  private final Object[] tuples_;
  private final boolean asc_;
  private final int lowerBound_;
  private final int upperBoundInclu_;

  private boolean hasNext_;
  private int nextPos_;

  BTreeLeafCursor(Comparator<? super K> comparator, Object[] tree, boolean asc) {
    this(comparator, tree, asc, 0, tupleSizeOfLeaf(tree) - 1);
  }

  BTreeLeafCursor(Comparator<? super K> comparator, Object[] tree, boolean asc, int lowerBound, int upperBoundInclu) {
    assert tree.length % 2 == 1 : "tree has to be a leaf";

    this.comparator_ = comparator;
    this.tuples_ = tree;
    this.asc_ = asc;
    this.lowerBound_ = lowerBound;
    this.upperBoundInclu_ = upperBoundInclu;
  }

  void rewind() {
    nextPos_ = asc_ ? lowerBound_ : upperBoundInclu_;
    hasNext_ = lowerBound_ <= nextPos_ && nextPos_ <= upperBoundInclu_;
  }
  
  T next() {
    if ( !hasNext_ ) throw new NoSuchElementException();

    T ret = (T) tuples_[nextPos_];
    nextPos_ += asc_ ? 1 : -1;
    hasNext_ = lowerBound_ <= nextPos_ && nextPos_ <= upperBoundInclu_;

    return ret;
  }

  T peek() {
    if ( !hasNext_ ) throw new NoSuchElementException();
    return (T) tuples_[nextPos_];
  }

  boolean hasNext() {
    return hasNext_;
  }

  private int search(K key) {
    int l = asc_ ? nextPos_ : lowerBound_;
    int u = asc_ ? upperBoundInclu_ : nextPos_;

    return Arrays.binarySearch((K[]) tuples_, l, u + 1, key, comparator_);
  }

  T next(K key) {

    if ( !hasNext_ ) throw new NoSuchElementException();

    if ( comparator_.compare((K) tuples_[nextPos_], key) == 0 ) {
      T ret = (T) tuples_[nextPos_];
      nextPos_ += asc_ ? 1 : -1;
      hasNext_ = lowerBound_ <= nextPos_ && nextPos_ <= upperBoundInclu_;
      return ret;
    }

    int find = search(key);
    if ( find >= 0 ) {
      T ret = (T) tuples_[find];
      nextPos_  = find + (asc_ ? 1 : -1);
      hasNext_ = lowerBound_ <= nextPos_ && nextPos_ <= upperBoundInclu_;
      return ret;
    } else {
      // If key is not found, nextPos_ stop at insert position of the key.
      int insert = -find - 1;
      nextPos_  = insert + (asc_ ? 0 : -1);
      hasNext_ = lowerBound_ <= nextPos_ && nextPos_ <= upperBoundInclu_;
      return null;
    }
  }
}