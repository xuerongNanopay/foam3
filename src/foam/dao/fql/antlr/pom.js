/**
 * @license
 * Copyright 2025 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.POM({
  name: 'ANTLR',

  setFlags: {
    web: false
  },

  files: [
    { name: 'ANTLR4' },
    { name: "FqlParser",       flags: "antlr4" },
  ]
});
