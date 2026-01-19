/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.util.btree;

public interface BTreeLoader<V> {

  void store(Object[] dest, int offset, int size);
  V next();

  public static class ArrayBTreeLoader<V> implements BTreeLoader<V> {
    
    private Object[] src;
    private int cursor;

    public void store(Object[] dest, int offset, int size) {
      System.arraycopy(src, cursor, dest, offset, size);
      cursor += size;
    }


    public V next()
    {
        return (V) src[cursor++];
    }
  }

    public static <V> ArrayBTreeLoader<V> of(Object[] src) {
    return of(src, 0);
  }

  public static <V> ArrayBTreeLoader<V> of(Object[] src, int offset) {
    ArrayBTreeLoader<V> ret = new ArrayBTreeLoader<>();
    ret.src = src;
    ret.cursor = offset;
    return ret;
  }
}