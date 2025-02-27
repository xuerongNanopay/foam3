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
  package: 'foam.u2.view',
  name: 'PasswordView',
  extends: 'foam.u2.View',

  requires: [
    'foam.u2.TextField'
  ],

  imports: [
    'theme?'
  ],

  css: `
    ^ {
      position: relative;
      width: 100%;
    }
    ^input-image {
      --fieldSize: $inputHeight;
      position: absolute;
      width: 16px;
      height: calc( var(--fieldSize) / 1.2);
      top: calc( var(--fieldSize) / 15);
      right: 1vh;
      opacity: 0.3;
    }
    .full-width-input-password:focus + ^input-image {
      opacity: 1;
    }
  `,

  constants: [
    {
      type: 'String',
      name: 'VISIBILITY',
      value: '/images/visibility.svg'
    },
    {
      type: 'String',
      name: 'VISIBILITY_OFF',
      value: '/images/visibility-off.svg'
    }
  ],

  properties: [
    {
      name: 'visibilityIcon',
      factory: function() {
        return this.VISIBILITY_OFF;
      }
    },
    {
      class: 'Boolean',
      name: 'passwordInvisible',
      value: true
    },
    {
      class: 'Boolean',
      name: 'passwordIcon'
    },
    {
      class: 'String',
      name: 'type',
      value: 'password'
    },
    'inputElement',
    {
      class: 'Boolean',
      name: 'isAvailable',
      value: true
    },
    {
      class: 'String',
      name: 'autocomplete'
    },
    {
      class: 'Boolean',
      name: 'validationEnabled',
      documentation: 'Should to set to false when entering old passwords and true when selecting a new one.',
      value: true
    },
    'name'
  ],

  methods: [
    function render() {
      this.SUPER();
      var typingTimer;
      var doneTypingInterval = 400;

      this.addClass()
        .start(this.TextField, {
          type$: this.type$,
          data$: this.data$,
          onKey: true,
          autocomplete: this.autocomplete
        })
          .addClass('full-width-input-password')
          .callIf(this.validationEnabled, function() {
            this.on('keyup', () => {
              clearTimeout(typingTimer);
              typingTimer = setTimeout(this.checkAvailability, doneTypingInterval);
            })
            .on('keydown', () => {
              clearTimeout(typingTimer);
              this.isAvailable = true;
            });
          })
          .attrs({ name: this.name$ })
        .end()

        .start('img')
          .show(this.passwordIcon$)
          .addClass(this.myClass('input-image'))
          .attr('src', this.visibilityIcon$)
          .on('mousedown', (e) => e.preventDefault())
          .on('click', () => this.visible())
        .end();
    },
    
    function fromProperty(p) {
      this.name = p.name;
      this.SUPER(p);
    },

    function visibleIcon(visibilityIcon, type) {
      this.visibilityIcon = visibilityIcon;
      this.type = type;
      this.passwordInvisible = ! this.passwordInvisible;
      this.enableClass('property-password', this.passwordInvisible);
    }
  ],

  listeners: [
    function visible() {
      if ( this.passwordInvisible ) {
        // Make password visible
        this.visibleIcon(this.VISIBILITY, 'text');
      } else {
        // Make password invisible
        this.visibleIcon(this.VISIBILITY_OFF, 'password');
      }
    },
    function checkAvailability() {
      this.theme?.passwordPolicy.validate(this.__context__, this.data)
        .then(x  => this.isAvailable = ! x)
        .catch(x => this.isAvailable = false);
    }
  ]
});
