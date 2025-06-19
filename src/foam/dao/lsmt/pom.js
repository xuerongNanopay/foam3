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
    javaFiles: [
    { name: "btree/COWBTree" },
    { name: "utils/BulkIterator" },
  ]
});