/**
 * @license
 * Copyright 2025 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */
foam.CLASS({
  package: 'foam.dao.fql.antlr',
  name: 'ANTLRCompiler',

  properties: [
    {
      name: 'antlr',
    },
  ],

  methods: [

  ]
});

foam.ANTLR = function(model) {
//   var compiler = foam.xsd.XSDCompiler.create(model);
//   // console.log('***************************************** XSD COMPILER ', model.xsdPath, model.files);
//   if ( compiler.xsdPath && compiler.files.length > 0 ) {
//     compiler.compileAll();
//   } else if ( compiler.xsd ) {
//     compiler.compile();
//   } else {
//     // console.log('****************************************************************** XSD ERROR');
//     throw new Error("compiler neither xsd or xsdPath set");
//   }
  console.log("hahah .moel: " + model.antlr);
};

foam.flags['antlr'] = true;