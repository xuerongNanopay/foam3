/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.POM({
  name: "lsmt",
  files: [
    { name: "LSMDAO", flags: "java" },
  ],

  files: [
    { name: "btree/BTreeTest",    flags: "java&test" },
    { name: "btree/BTreeObject",  flags: "java" },
  ],

  javaFiles: [
    { name: "btree/BTree" },
    { name: "btree/BTreeUpdate" },
    { name: "btree/BTreeOutputter" },
    { name: "btree/BTreeCursor" },
    { name: "btree/BTreeFullCursor" },
    { name: "btree/BTreeLeafCursor" },
    { name: "btree/ZoneMap" },
    { name: "btree/BTreeIndex" },
    { name: "utils/BulkIterator" },
    { name: "utils/ArrayUtil" },
  ]
});