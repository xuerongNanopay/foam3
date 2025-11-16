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
  ],

  methods: [
    {
      name: 'runTest',
      javaCode: `
        var btree = BTree.empty();
        System.out.println(BTreeOutputter.stringify(btree));

        int len = 32768;
        var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
        btree = BTree.build(bi, len);
        // System.out.println(BTreeOutputter.stringify(btree));
        System.out.println("FFFF: " + BTree.find(btree, 11, Integer::compare));
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
        int len = 32768;
        var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
      `
    }
  ]
})