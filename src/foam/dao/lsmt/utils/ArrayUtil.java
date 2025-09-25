/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.lsmt.utils;

public class ArrayUtil {

  public static void reverse(Object[] array, int from, int to) {
    int mid = (from + to) / 2;
    for ( int i = from ; i < mid ; i++ ) {
      int j = to - (1 + i - from);
      Object tmp = array[i];
      array[i] = array[j];
      array[j] = tmp;
    }
  }

}