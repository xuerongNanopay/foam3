/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */
package foam.poseidon;

public class Poseidon {
  static {
    // System.loadLibrary("libfoam_poseidon");
    System.load("/Users/xuerong/workspace/foam3/src/foam/objectdb/foam_poseidon/target/release/libfoam_poseidon.dylib");
  }
  
  public static native int addNumbers(int a, int b);
  public static native int mulNumbers(int a, int b);

  static public void say() {
    System.out.println("This is Poseidon: " + mulNumbers(2, 2));
  }

  public static void main(String[] args) {
    say();
  }
}