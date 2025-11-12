/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.dao.lsmt.btree',
  name: 'BTreeTest',
  extends: 'foam.core.test.Test',

  methods: [
    {
      name: 'runTest',
      javaCode: `
        var btree = BTree.empty();
        System.out.println(BTreeOutputter.stringify(btree));
      `
    }
  ]
})