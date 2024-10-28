/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.nanos.notification',
  name: 'NotificationWidget',
  extends: 'foam.u2.View',

  imports: [
    'pushMenu'
  ],

  css: `
    ^ {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      height: 100%;
      gap: 1rem;
    }
    ^text-container {
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
    ^button-container {
      display: flex;
      gap: 1rem;
    }
  `,

  messages: [
    { name: 'TITLE', message: 'Stay up to date' },
    { name: 'TEXT', message: 'We use on device notifications update you.' },
  ],

  methods: [
    function render() {
      this.start().addClass(this.myClass())

        .start().addClass(this.myClass('text-container'))
          .start().addClass('h400')
              .add(this.TITLE)
          .end()
          .add(this.TEXT)
        .end()
        
        .start().addClass(this.myClass('button-container'))
          .start(this.ENABLE_NOTIF, { buttonStyle: 'PRIMARY' })
              .addClass(this.myClass('button'))
          .end()
          .start(this.DONT_SHOW, { buttonStyle: 'LINK' })
              .addClass(this.myClass('button'))
          .end()
        .end()

      .end();
    }
  ],

  actions: [
    {
      name: 'enableNotif',
      label: 'Enable Notifications',
      code: function(X) {
        this.pushMenu('notification-settings');
      }
    },
    {
      name: 'dontShow',
      label: 'Dont Show Again',
      code: function(X) {
        // fill me in
      }
    }
  ]
});

