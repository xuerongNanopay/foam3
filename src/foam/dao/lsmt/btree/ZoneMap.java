/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

class ZoneMap {
  int[] presum;

  ZoneMap(int[] presum) {
    // record the number of tuple up to current child.
    this.presum = presum;
  }
 
  int size() {
    return presum[presum.length-1];
  }

  int sizeOfChildAt(int i) {
    return presum[i];
  }
}