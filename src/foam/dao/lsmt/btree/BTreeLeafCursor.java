/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

import java.util.NoSuchElementException;

public class BTreeLeafCursor<T> {

  private final boolean forward;
  private final int from, to;
  private final T[] btree;
  private int nextPos;
  private boolean hasNext;

  public BTreeLeafCursor(Object[] btree, int from, int to, boolean isDESC) {
    this.from = from;
    this.to = to;
    this.forward = !isDESC;
    this.btree = (T[]) btree;
    rewind();
  }

  public boolean hasNext() {
    return hasNext;
  }

  public void rewind() {
    nextPos = forward ? from : to;
    hasNext = nextPos >= from && nextPos <= to;
  }

  public T peek() {
    if ( !hasNext )
      throw new NoSuchElementException();
    return btree[nextPos];
  }

  public T next() {
    if ( !hasNext )
      throw new NoSuchElementException();

    T ret = btree[nextPos];
    nextPos += forward ? 1 : -1;
    hasNext = nextPos >= from && nextPos <= to;
    return ret;
  }
  
}