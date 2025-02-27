/**
 * @license
 * Copyright 2016 Google Inc. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

foam.CLASS({
  package: 'foam.u2',
  name: 'Autocompleter',

  documentation: `Basic autocomplete controller. Supports simple
    autocomplete, defaulting to querying by Keyword. Use this as a base
    class for other, more sophisticated autocompleters.`,

  properties: [
    {
      name: 'dao',
      required: true,
      documentation: 'The DAO to complete against.'
    },
    {
      class: 'String',
      name: 'partial',
      documentation: 'The string the user has entered so far. Usually bound to some text field.'
    },
    {
      name: 'queryFactory',
      documentation: 'Turns the user\'s string into an mLang query. Defaults to Keyword.',
      value: function(str) {
        return foam.mlang.predicate.Keyword.create({ arg1: str });
      }
    },
    {
      class: 'foam.dao.DAOProperty',
      name: 'filteredDAO',
      factory: function() {
        return this.dao;
      }
    },
    {
      class: 'Boolean',
      name: 'loading'
    }
  ],

  methods: [
    function init() {
      this.SUPER();
      this.partial$.sub(this.onUpdate);
      this.onUpdate();
    }
  ],

  listeners: [
    {
      name: 'onUpdate',
      isFramed: true,
      code: function onUpdate() {
        this.loading = true;
        if ( ! this.dao ) return;
        this.filteredDAO = this.partial ?
          this.dao.where(this.queryFactory(this.partial)) :
          this.dao;
        this.loading = false;
      }
    }
  ]
});
