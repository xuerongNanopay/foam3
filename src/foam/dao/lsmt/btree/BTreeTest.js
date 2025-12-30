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
    'java.util.Random',
    'foam.dao.index.*',
    'static foam.dao.lsmt.btree.BTree.*'
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
        // testInsertAndFind(x);
        // benchMark(x);
        // btreeDebug(x);

        for ( int i = 0 ; i < 50 ; i++ ) {
          verifyAAtree(1000000, i);
          verifyBtree(1000000, i);
        }
      `
    },
    {
      name: 'btreeDebug',
      args: 'Context x',
      javaCode: `
        /**
         * max: 31, min: 15
         * - 0
         * - 1 - 31
         * - 32 - 63
         * - 558
         */ 
      
        int size = 0;
        var batch = generateIntegerArray(size);
        var btree = insertToBTree(batch);
        System.out.println("size: " + size + "\\n" + BTreeOutputter.stringify(btree));

        size = 1;
        batch = generateIntegerArray(size);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));

        size = 31;
        batch = generateIntegerArray(size);
        // batch = generateIntegerArrayReverse(size);
        // shuffleArray(batch, 43);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));

        size = 32;
        // batch = generateIntegerArray(size);
        batch = generateIntegerArrayReverse(size);
        shuffleArray(batch, 64);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));

        size = 47;
        batch = generateIntegerArray(size);
        // batch = generateIntegerArrayReverse(size);
        // shuffleArray(batch, 23);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));

        size = 255;
        // batch = generateIntegerArray(size);
        batch = generateIntegerArrayReverse(size);
        shuffleArray(batch, 46);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));

        size = 558;
        batch = generateIntegerArray(size);
        batch = generateIntegerArrayReverse(size);
        // shuffleArray(batch, 46);
        btree = insertToBTree(batch);
        System.out.println("size: " + BTree.size(btree) + "\\n" + BTreeOutputter.stringify(btree));
      `
    },
    {
      name: 'insertToBTree',
      args: 'Object[] inserts',
      type: 'Object[]',
      javaCode: `
        var btree = BTree.empty();
        BTreeUpdate.SimpleUpdate<Integer> intUpdater = BTreeUpdate.SimpleUpdate.of((o, n) -> n);
        for ( int i = 0 ; i < inserts.length ; i++ ) {
          btree = BTreeUpdate.merge(Integer::compare, intUpdater, btree, BTree.singleton(inserts[i]));
        }
        return btree;
      `
    },
    {
      name: 'generateTestObjects',
      type: 'BTreeObject[]',
      args: 'int size',
      javaCode: `
        var ret = new BTreeObject[size];
        for ( int i = 0 ; i < size ; i++ ) {
          ret[i] = new BTreeObject(i);
        }
        return ret;
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
      name: 'generateIntegerArrayReverse',
      type: 'Object[]',
      args: 'int length',
      javaCode: `
        var ret = new Object[length];
        for ( int i = 0 ; i < length ; i++ ) {
          ret[i] = length - i;
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
            var equal = Objects.equals(i, BTree.find(Integer::compare, btree, i));
            if ( !equal ) {
              test(equal, String.format("%d is not found in tree.", i));
              break;
            }
          } else {
            var equal = Objects.equals(null, BTree.find(Integer::compare, btree, i));
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
        int len = 100000;
        var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
        var btree = BTree.build(bi, len);
        var ret = true;

        // Runtime.getRuntime().gc();

        long start = System.nanoTime();
        for ( int i = 1 ; i <= len ; i++ ) {
          var find = BTree.find(Integer::compare, btree, i);
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
    },
    {
      name: 'testUpdateAndFind',
      args: 'Context x',
      javaCode: `
        int len = 100000;
        var arrs = generateIntegerArray(len);


      `
    },
    {
      name: 'benchMark',
      args: 'Context x',
      javaCode: `
        long start = 0;
        long end = 0;
        long elapsedNanos = 0;
        long elapsedMillis = 0;

        int size = 1000000;
        var testObjs = generateTestObjects(size);
        shuffleArray(testObjs, 64);

        var treeIndex = new TreeIndex(BTreeObject.ID, true);
        Object treeState = null;

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          treeState = treeIndex.put(treeState, testObjs[i]);
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark AATree index insert, Elapsed: " + elapsedMillis + " ms");

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          if ( treeIndex.find(treeState, testObjs[i].getId()) == null ) {
            throw new RuntimeException("AAAAA11");
          }
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark AATree index find, Elapsed: " + elapsedMillis + " ms");

        var btreeIndex = new BTreeIndex(BTreeObject.ID);
        Object btreeStatus = null;

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          btreeStatus = btreeIndex.put(btreeStatus, testObjs[i]);
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark BTree index insert, Elapsed: " + elapsedMillis + " ms");

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          if ( btreeIndex.find(btreeStatus, testObjs[i].getId()) == null ) {
            throw new RuntimeException("hahahh: " + i + ", " + testObjs[0]);
          }
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark BTree index find, Elapsed: " + elapsedMillis + " ms");

      `
    },
    {
      name: 'verifyAAtree',
      args: 'int size, int seed',
      javaCode: `
        var testObjs = generateTestObjects(size);
        shuffleArray(testObjs, seed);

        long start = 0;
        long end = 0;
        long elapsedNanos = 0;
        long elapsedMillis = 0;

                var treeIndex = new TreeIndex(BTreeObject.ID, true);
        Object treeState = null;

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          treeState = treeIndex.put(treeState, testObjs[i]);
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark AATree index insert, Elapsed: " + elapsedMillis + " ms, size: " + size + ", seed: " + seed);

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          if ( treeIndex.find(treeState, testObjs[i].getId()) == null ) {
            throw new RuntimeException("AAAAA11");
          }
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark AATree index find, Elapsed: " + elapsedMillis + " ms, size: " + size + ", seed: " + seed);
      `
    },
    {
      name: 'verifyBtree',
      args: 'int size, int seed',
      javaCode: `
        var testObjs = generateTestObjects(size);
        shuffleArray(testObjs, seed);

        long start = 0;
        long end = 0;
        long elapsedNanos = 0;
        long elapsedMillis = 0;

        var btreeIndex = new BTreeIndex(BTreeObject.ID);
        Object btreeStatus = null;

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          btreeStatus = btreeIndex.put(btreeStatus, testObjs[i]);
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark BTree index insert, Elapsed: " + elapsedMillis + " ms, size: " + size + ", seed: " + seed);

        start = System.nanoTime();
        for ( int i = 0 ; i < size ; i++ ) {
          if ( btreeIndex.find(btreeStatus, testObjs[i].getId()) != testObjs[i]) {
            System.out.println(BTreeOutputter.stringify((Object[]) btreeStatus));
            throw new RuntimeException("" + testObjs[i] + " no found with size: " + size + ", seed: " + seed);
          }
        }
        end = System.nanoTime();
        elapsedNanos = end - start;
        elapsedMillis = elapsedNanos / 1_000_000;
        test(true, "benchMark BTree index find, Elapsed: " + elapsedMillis + " ms, size: " + size + ", seed: " + seed + ", height: " + doVerifyBtreeDepth((Object[]) btreeStatus));

      `
    },
    {
      name: 'shuffleArray',
      args: 'Object[] a, int randSeed',
      javaCode: `
        Random rnd = new Random(randSeed);

        for (int i = a.length - 1; i > 0; i--) {
          int j = rnd.nextInt(i + 1);
          Object tmp = a[i];
          a[i] = a[j];
          a[j] = tmp;
        }
      `
    },
  ],
  javaCode: `
    int doVerifyBtreeDepth(Object[] btree) {
      int h = testHeight(btree);
      return h;
      // return h > 0 && h <= 6 ? true : false;
    }
    int testHeight(Object[] btree) {
      if ( isLeaf(btree) ) return 1;

      int childOffset = firstChildOfInternal(btree);
      int childSize = childOffset + 1;

      int cHeight = testHeight((Object[]) btree[childOffset]);

      if ( cHeight == -1 ) return cHeight;

      for ( int i = 1 ; i < childSize ; i++ ) {
        int h = testHeight((Object[]) btree[childOffset + i]);
        if ( h == -1 || h != cHeight ) return -1;
      }

      return cHeight + 1;
    }

    boolean verifyLeafTuple(Object[] leaf) {
      if ( (leaf.length & 1) != 1 ) return false;
      if ( sizeOfLeaf(leaf) < MIN_TUPLES ) return false;
      if ( sizeOfLeaf(leaf) > MAX_TUPLES ) return false;

      return true;
    }

    boolean verifyInternalTuple(Object[] internal) {
      if ( tupleSizeOfInternal(internal) < MIN_TUPLES ) return false;
      if ( tupleSizeOfInternal(internal) > MAX_TUPLES ) return false;

      return true;
    }

    boolean verifyBTreeTuple(Object[] btree) {
      if ( isLeaf(btree) ) verifyLeafTuple(btree);

      var ret = verifyInternalTuple(btree);
      if ( ret == false ) return false;

      int childOffset = firstChildOfInternal(btree);
      int childSize = childOffset + 1;

      for ( int i = 0 ; i < childSize ; i++ ) {
        ret = verifyBTreeTuple((Object[]) btree[childOffset + i]);
        if ( ret == false ) return false;
      }

      return true;
    }
  `
})