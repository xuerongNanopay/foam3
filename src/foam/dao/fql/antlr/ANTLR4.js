/**
 * @license
 * Copyright 2025 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */
foam.CLASS({
  package: 'foam.dao.fql.antlr',
  name: 'ANTLR4Compiler',

  properties: [
    'package',
    'name',
    'antlr'
  ],

  methods: [
    function compile() {
      const { execSync } = require('child_process');
      const fs = require('fs');
      const path = require('path');

      const antlr4Dir = X.builddir + '/antlr4/' + this.package.replaceAll('.', '_');
      fs.mkdirSync(antlr4Dir, { recursive: true });

      const antlr4Filename = this.name + '.g4';
      const filePath = path.join(antlr4Dir, antlr4Filename);

      fs.writeFileSync(filePath, this.antlr.trim());
      console.log(`${antlr4Dir + '/' + this.package.replaceAll('.', '_')}`);
      execSync(`antlr4 ${filePath}`);

      const javaPath = path.join(X.builddir, 'src/java', this.package.replaceAll('.', '/'))
      fs.mkdirSync(javaPath, { recursive: true });

      const javaFiles = fs.readdirSync(antlr4Dir);

      console.log("aaaaa", X.javaFiles)
      for ( const file of javaFiles ) {
        if ( file.endsWith('.java') ) {
          const srcPath = path.join(antlr4Dir, file);
          const destPath = path.join(javaPath, file);
          fs.renameSync(srcPath, destPath);
          const outFile = X.outdir + path.sep + this.package.replaceAll(/\./g, path.sep) + "/" + file;
          X.javaFiles.push(outFile);
        }
      }
    },
  ]
});

foam.ANTLR4 = function(model) {
  var compiler = foam.dao.fql.antlr.ANTLR4Compiler.create(model);
  compiler.compile();
};

foam.flags['antlr4'] = true;