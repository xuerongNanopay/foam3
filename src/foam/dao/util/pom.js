/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.POM({
  name: "util",

  files: [
    { name: "btree/BTreeTest",    flags: "java&test" },
    { name: "btree/BTreeObject",  flags: "java" },
  ],

  javaFiles: [
    { name: "btree/BTree" },
    { name: "btree/BTreeUpdate" },
    { name: "btree/BTreeRemove" },
    { name: "btree/BTreeOutputter" },
    { name: "btree/BTreeIndex" },
    { name: "btree/BTreeLoader" },
    { name: "btree/BTreeLeafCursor" },
  ]
});