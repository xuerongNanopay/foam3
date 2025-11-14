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

        int len = 10;
        var bi = BulkIterator.<Integer>of(generateIntegerArray(len));
        btree = BTree.build(bi, len);
        System.out.println(BTreeOutputter.stringify(btree));

      `
    },
    {
      name: 'generateIntegerArray',
      type: 'Object[]',
      args: 'int length',
      javaCode: `
        var ret = new Object[length];
        for ( int i = 0 ; i < length ; i++ ) {
          ret[i] = i;
        }
        return ret;
      `
    }
  ]
})