/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

package foam.dao.util.btree;

import static foam.dao.util.btree.BTree.*;

public class BTreeOutputter {

  public static String stringify(Object[] btree) {

    if ( isEmpty(btree) ) return "(T:L | S:0 | H:0): []";

    StringBuilder sb = new StringBuilder();

    outputNode(sb, btree, 0, "");
    return sb.toString();

  }

  public static void outputNode(StringBuilder sb, Object[] node, int depth, String prefix) {

    var indent = " ".repeat(depth);

    if ( isLeaf(node) ) {
      int leafSize = sizeOfLeaf(node);

      sb.append(prefix).append(indent);
      sb.append(outputNodeMeta(node)).append(": [ ");
      for ( int i = 0 ; i < leafSize ; i++ ) {
        sb.append(node[i]);
        if ( i != leafSize-1 ) sb.append(", "); 
      }
      sb.append(" ]\n");
    } else {
      int tupleSize = tupleSizeOfInternal(node);
      int childSize = tupleSize+1;

      sb.append(prefix).append(indent);
      sb.append(outputNodeMeta(node));
      sb.append(": [ ");
      for ( int i = 0 ; i < tupleSize ; i++ ) {
        sb.append(node[i]);
        if ( i != tupleSize-1 ) sb.append(", ");
      }
      sb.append(" ]\n");

      for ( int i = 0 ; i < childSize ; i++ ) {
        var child = (Object[]) node[tupleSize+i];
        var carryOverPrefix = prefix + indent;
        var metaPrefix = i == childSize-1 ? "┗ " : "┣ ";
        var childPrefix = i == childSize-1 ? "  " : "┃ ";

        sb.append(carryOverPrefix)
          .append(metaPrefix);

        if ( i == 0 ) {
          sb.append("(⤝, " + node[i] + ")\n");
        } else if ( i == childSize-1) {
          sb.append("(" + node[i-1] + ", ⤠)\n");
        } else {
          sb.append("(" + node[i-1] + ", " + node[i] + ")\n");
        }

        outputNode(sb, child, depth+1, carryOverPrefix + childPrefix);
      }
    }
  }

  private static String outputNodeMeta(Object[] node) {
    if ( isLeaf(node) ) {
      return String.format("<S:%,d | H:%d>", size(node), height(node));
    } else {
      return String.format("<T:%d | | S:%,d | H:%d>", tupleSizeOfInternal(node), size(node), height(node));
    }
  }
}