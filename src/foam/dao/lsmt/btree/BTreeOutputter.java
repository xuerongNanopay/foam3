/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.lsmt.btree;

import static foam.dao.lsmt.btree.BTree.*;

public class BTreeOutputter {

  public static String stringify(Object[] btree) {

    if ( isEmpty(btree) ) return "(T:L | S:0 | H:0): { }";

    StringBuilder sb = new StringBuilder();

    return sb.toString();

  }

  public static void outputNode(StringBuilder sb, Object[] node, int height, String prefix) {

    if ( isLeaf(node) ) {
      int leafSize = sizeOfLeaf(node);

      sb.append("L: [ ");
      for ( int i = 0 ; i < leafSize ; i++ ) {
        sb.append(node[i]);
        if ( i != leafSize-1 ) sb.append(", "); 
      }
      sb.append(" ]\n");
    } else {
      sb.append("TODO");
    }
  }
}