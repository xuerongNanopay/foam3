/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.dao.lsmt.btree',
  name: 'BTreeTest',
  extends: 'foam.core.test.Test',

  javaImports: [
    'foam.dao.lsmt.utils.*',
    'java.util.Objects',
  ],

  methods: [
    {
      name: 'runTest',
      javaCode: `
        // var btree = BTree.empty();
        // System.out.println(BTreeOutputter.stringify(btree));

        // int len = 32768;
        // var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
        // btree = BTree.build(bi, len);
        // // System.out.println(BTreeOutputter.stringify(btree));
        // System.out.println("FFFF: " + BTree.find(btree, 11, Integer::compare));
        
        // test1(x);
        testInsertAndFind(x);
      `
    },
    {
      name: 'generateIntegerArray',
      type: 'Object[]',
      args: 'int length',
      javaCode: `
        var ret = new Object[length];
        for ( int i = 0 ; i < length ; i++ ) {
          ret[i] = i+1;
        }
        return ret;
      `
    },
    {
      name: 'test1',
      args: 'Context x',
      javaCode: `

        int size = 32768;
        var ret = new Object[size];
        for ( int i = 0 ; i < size ; i++ ) {
          ret[i] = i*2;
        }
        var patch = BulkIterator.<Integer>of(ret);
        var btree = BTree.build(patch, size);
        // System.out.println(BTreeOutputter.stringify(btree));
        
        for ( int i = 0 ; i < size*2 ; i++ ) {
          if ( i%2 == 0 ) {
            var equal = Objects.equals(i, BTree.find(btree, i, Integer::compare));
            if ( !equal ) {
              test(equal, String.format("%d is not found in tree.", i));
              break;
            }
          } else {
            var equal = Objects.equals(null, BTree.find(btree, i, Integer::compare));
            if ( !equal ) {
              test(equal, String.format("%d should't be in tree.", i));
              break;
            }
          }

          if ( i == size*2-1) {
            test(true, "BTree test1 success");
          }
        }

      `
    },
    {
      name: 'testInsertAndFind',
      args: 'Context x',
      javaCode: `
        int len = 1000000;
        var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
        var btree = BTree.build(bi, len);
        var ret = true;

        // Runtime.getRuntime().gc();

        long start = System.nanoTime();
        for ( int i = 1 ; i <= len ; i++ ) {
          var find = BTree.find(btree, i, Integer::compare);
          if ( find == null || find != i ) {
            test(false, String.format("%d is not found in BTree.", i));
            ret = false;
            break;
          }
        }
        
        long end = System.nanoTime();
        long elapsedNanos = end - start;
        long elapsedMillis = elapsedNanos / 1_000_000;

        if ( ret ) {
          test(ret, "BTree testInsertAndFind success, Elapsed: " + elapsedMillis + " ms");
        }
      `
    }
  ]
})