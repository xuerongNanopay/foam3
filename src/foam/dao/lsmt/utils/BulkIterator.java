/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.lsmt.utils;

public interface BulkIterator<V> {

  void load(Object[] dest, int offset, int size);
  V next();

  public static class ArrayIterator<V> implements BulkIterator<V> {
    
    private Object[] src;
    private int cursor;

    public void load(Object[] dest, int offset, int size) {
      System.arraycopy(src, cursor, dest, offset, size);
      cursor += size;
    }


    public V next()
    {
        return (V) src[cursor++];
    }
  }
}