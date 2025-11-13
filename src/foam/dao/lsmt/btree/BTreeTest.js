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

        var bi = BulkIterator.<Integer>of(generateIntegerArray(5));
        btree = BTree.build(bi, 5);
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