/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 *     http://www.apache.org/licenses/LICENSE-2.0
 */

foam.INTERFACE({
  package: 'foam.nanos.ruler',
  name: 'RulePredicate',
  documentation: 'Interface for a Predicate which takes both the old and new objects, and a Context.',

  methods: [
    {
      name: 'ruleF',
      type: 'Boolean',
      args: 'Context x, foam.core.FObject o, foam.core.FObject n'
    }
  ]
});


foam.CLASS({
  name: 'AddRulePredicateToPredicateRefine',
  refines: 'foam.mlang.predicate.Predicate',

  implements: [ 'foam.nanos.ruler.RulePredicate' ]
});


foam.CLASS({
  name: 'AddRulePredicateToAbstractPredicateRefine',
  refines: 'foam.mlang.predicate.AbstractPredicate',

  methods: [
    {
      name: 'ruleF',
      type: 'Boolean',
      args: 'Context x, foam.core.FObject o, foam.core.FObject n',
      code: 'return this.f(n);',
      javaCode: 'return f(x.put("OLD", o).put("NEW", n));'
    }
  ]
});


foam.CLASS({
  name: 'AddRulePredicateToAndRefine',
  refines: 'foam.mlang.predicate.And',

  methods: [
    {
      name: 'ruleF',
      javaCode: `
        for ( int i = 0 ; i < getArgs().length ; i++ ) {
          if ( ! getArgs()[i].ruleF(x, o, n) ) return false;
        }
        return true;
      `
    }
  ]
});


foam.CLASS({
  name: 'AddRulePredicateToOrRefine',
  refines: 'foam.mlang.predicate.Or',

  methods: [
    {
      name: 'ruleF',
      javaCode: `
        for ( int i = 0 ; i < getArgs().length ; i++ ) {
          if ( getArgs()[i].ruleF(x, o, n) ) return true;
        }
        return false;
      `
    }
  ]
});


foam.CLASS({
  name: 'AddRulePredicateToNotRefine',
  refines: 'foam.mlang.predicate.Not',

  methods: [
    {
      name: 'ruleF',
      javaCode: 'return ! getArg1().ruleF(x, o, n);'
    }
  ]
});


foam.CLASS({
  package: 'foam.nanos.ruler',
  name: 'RulePredicateAxiom',

  documentation: 'An Axiom for defining Rule Predicates.',

  properties: [
    'name',
    {
      class: 'Class',
      name: 'of'
    },
    {
      class: 'Boolean',
      name: 'strict'
    },
    {
      name: 'properties',
      factory: function() { return []; }
    },
    { name: 'javaCode', required: false },
    'documentation'
  ],

  methods: [
    function installInClass(cls) {
      // RulePredicates are Java-only, so just record the class
      this.of = cls;
    },

    function buildJavaClass(cls) {
      var strictCheck = this.strict ?
        'if ( n.getClass() != getOf().getObjClass() ) return false;' :
        '' ;

      var javaCode = `
        try {
          var o = (${this.of.id}) o_;
          var n = (${this.of.id}) n_;
          ${strictCheck}
          ${this.javaCode}
        } catch (ClassCastException e) {
          return false;
        }
      `;

      var model = {
        name: this.name,
        extends: 'foam.mlang.predicate.AbstractPredicate',
//        implements: [ 'foam.nanos.ruler.RulePredicate' ],
        methods: [
          {
            name: 'ruleF',
            args: 'foam.core.X x, FObject o_, FObject n_',
            javaCode: javaCode
          }
        ]
      };

      // TODO: add ability to specify not to generate Builder

      if ( this.properties.length ) {
        model.javaGenerateConvenienceConstructor = true;
        model.properties = this.properties;
      } else {
        model.javaCode = `
          private static ${this.name} instance__ = new ${this.name}();
          public  static ${this.name} instance() { return instance__; }
        `;
      }

      foam.core.InnerClass.create({model: model}).buildJavaClass(cls);
      /*
      cls.constant({
        name: foam.String.constantize(this.name),
        type: 'foam.nanos.ruler.RulePredicate',
        value: ${javaCode}
      });
      */
    }
  ]
});

/*

foam.CLASS({
  refines: 'foam.core.Model',
  package: 'foam.core',
  name: 'ModelConstantRefine',
  properties: [
    {
      class: 'AxiomArray',
      of: 'Constant',
      name: 'constants',
      adapt: function(_, a, prop) {
        if ( ! a ) return [];
        if ( ! Array.isArray(a) ) {
          var cs = [];
          for ( var key in a ) {
            cs.push(foam.core.Constant.create({name: key, value: a[key]}));
          }
          return cs;
        }
        var b = new Array(a.length);
        for ( var i = 0 ; i < a.length ; i++ ) {
          b[i] = prop.adaptArrayElement.call(this, a[i], prop);
        }
        return b;
      }
    }
  ]
});
*/
