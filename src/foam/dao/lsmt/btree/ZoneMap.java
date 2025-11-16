/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

class ZoneMap {
  int[] preSum;

  ZoneMap(int[] preSum) {
    // record the number of tuple up to current child.
    this.preSum = preSum;
  }
 
  int size() {
    return preSum[preSum.length-1];
  }

  int sizeOfChildAt(int i) {
    return preSum[i];
  }
}