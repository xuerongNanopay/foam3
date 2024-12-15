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
  name: 'AddRulePredicateToAbstractPredicateRefine',
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
