/**
 * @license
 * Copyright 2015 Google Inc. All Rights Reserved.
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
  package: 'foam.u2.tag',
  name: 'Input',
  extends: 'foam.u2.View',

  css: `
    /* Still show outline when focused as read-only to help accessibility */
    ^:read-only:focus { outline: 1px solid rgb(238, 238, 238); }
  `,

  properties: [
    [ 'nodeName', 'input' ],
    {
      name: 'data',
      preSet: function(o, d) {
        var f = ! d || typeof d === 'string' || typeof d === 'number' || typeof d === 'boolean' || foam.Date.isInstance(d);
        if ( ! f ) {
          this.__context__.warn('Set Input data to non-primitive:' + d);
          return o;
        }
        return d;
      }
      /*
      assertValue: function(d) {
        foam.assert(! d || typeof d === 'string' || typeof d === 'number' || typeof d === 'boolean' || foam.Date.isInstance(d), 'Set Input data to non-primitive.');
      }*/
    },
    {
      class: 'Boolean',
      name: 'onKey',
      attribute: true,
      // documentation: 'When true, $$DOC{ref:".data"} is updated on every keystroke, rather than on blur.'
    },
    {
      class: 'Boolean',
      name: 'autofocus',
      attribute: true,
      documentation: 'If enabled, field gains focus when added to screen.'
    },
    {
      class: 'Int',
      name: 'size'
    },
    {
      class: 'Int',
      name: 'maxLength',
      attribute: true,
      // documentation: 'When set, will limit the length of the input to a certain number'
    },
    'type',
    'placeholder',
    'ariaLabel',
    [ 'autocomplete', true ],
//    'autocompleter',
    'inputMode', // Allows a browser to display an appropriate virtual keyboard
    {
      name: 'choices',
      documentation: `Array of [value, text] choices. You can pass in just
          an array of strings, which are expanded to [str, str]. Can also
          be a map, which results in [key, value] pairs listed in
          enumeration order.`,
      factory: function() { return []; },
      adapt: function(old, nu) {
        if ( typeof nu === 'object' && ! Array.isArray(nu) ) {
          var out = [];
          for ( var key in nu ) {
            if ( nu.hasOwnProperty(key) ) out.push([ key, nu[key] ]);
          }
          if ( this.dynamicSize ) {
            this.size = Math.min(out.length, this.maxSize);
          }
          return out;
        }

        nu = foam.Array.clone(nu);

        // Upgrade single values to [value, value].
        for ( var i = 0; i < nu.length; i++ ) {
          if ( ! Array.isArray(nu[i]) ) {
            nu[i] = [ nu[i], nu[i] ];
          }
        }

        if ( this.dynamicSize ) this.size = Math.min(nu.length, this.maxSize);
        return nu;
      }
    }
  ],

  methods: [
    function render() {
      this.SUPER();
      var self = this;

      if ( this.size          ) this.setAttribute('size',        this.size);
      if ( this.type          ) this.setAttribute('type',        this.type$);
      if ( this.placeholder   ) this.setAttribute('placeholder', this.placeholder);
      if ( this.ariaLabel     ) this.setAttribute('aria-label',  this.ariaLabel);
      if ( this.maxLength > 0 ) this.setAttribute('maxlength',   this.maxLength);
      if ( this.inputMode     ) this.setAttribute('inputmode',   this.inputMode);
      this.setAttribute('autocomplete', this.autocomplete ?
        (foam.String.isInstance(this.autocomplete) ? this.autocomplete : 'on') :
        'off'
      );
      if ( this.choices && this.choices.length ) {
        this.
          setAttribute('list', this.id + '-choices').
          start('datalist').
            setID(self.id + '-choices').
            forEach(this.choices, function(c) {
              var key   = c[0];
              var label = c[1];
              this.start('option').attrs({value: key}).add(label).end();
            }).
          end();
      } /* Was for compatibility with foam.u2.view.TextField, which no longer exists.
      else if ( this.autocompleter ) {
        this.
          setAttribute('list', this.id + '-autocomplete').
          start('datalist').
            setID(self.id + '-autocomplete').
            select(self.autocompleter.dao, function(o) {
              var label = o.label;
              this.start('option').add(label).end();
            }).
          end();
      } */

      this.initCls();
      this.link();
    },

    function initCls() {
      // Template method, can be overriden by sub-classes
      this.addClass();
    },

    function link() {
      // Template method, can be overriden by sub-classes
      this.attrSlot(null, this.onKey ? 'input' : null).linkFrom(this.data$);
    },

    function fromProperty(p) {
      this.SUPER(p);

      if ( ! this.hasOwnProperty('onKey') ) {
        this.onKey = p.hasOwnProperty('onKey') ? p.onKey : p.validateObj || p.internalValidateObj;
      }
      if ( ! this.hasOwnProperty('maxLength') && p.maxLength ) this.maxLength = p.maxLength;
      this.ariaLabel = p.label || p.name;

      if ( p.normalize ) {
        this.on('blur', () => this.data$.set(p.normalize(this.data$.get(), p)));
      }
    },

    function updateMode_(mode) {
      // TODO: make sure that DOM is updated if values don't change
      this.setAttribute('readonly', mode === foam.u2.DisplayMode.RO);
      this.setAttribute('disabled', mode === foam.u2.DisplayMode.DISABLED);
      this.show(mode !== foam.u2.DisplayMode.HIDDEN);
    }
  ]
});
