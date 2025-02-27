/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.box',
  name: 'SessionReplyBox',
  extends: 'foam.box.ProxyBox',

  requires: [
    'foam.box.RPCErrorMessage'
  ],

  imports: [
    'auth?',
    'ctrl',
    'group',
    'loginSuccess?',
    'requestLogin?',
    'sessionTimer',
    'subject',
    'window'
  ],

  messages: [
    {
      name: 'REFRESH_MSG',
      message: 'Your session has expired. The page will now be refreshed so that you can log in again.',
    }
  ],

  properties: [
    {
      class: 'FObjectProperty',
      name: 'msg',
      type: 'foam.box.Message'
    },
    {
      class: 'FObjectProperty',
      name: 'clientBox',
      type: 'foam.box.Box'
    }
  ],

  methods: [
    {
      name: 'send',
      code: async function send(msg) {
        var self = this;
        if (
          this.RPCErrorMessage.isInstance(msg.object) &&
          msg.object.data.id === 'foam.nanos.auth.AuthenticationException'
        ) {
          if (!this.auth$) {
            return;
          }
          // If the user is already logged in when this happens, then we know
          // that something occurred on the backend to destroy this user's
          // session. Therefore we reset the client state and ask them to log
          // in again.
          var promptlogin = await this.auth.check(null, 'auth.promptlogin');
          var authResult  = await this.auth.check(null, '*');

          if ( this.loginSuccess && ( ! promptlogin || authResult ) ) {
            if ( this.ctrl ) this.ctrl.remove();
            // Set loginSuccess to false so that if multiple requests are sent with no authentication, alert is called only once
            this.loginSuccess = false;
            alert(this.REFRESH_MSG);
            (this.window || window).location.reload();
            return;
          }

          this.requestLogin().then(function() {
            self.clientBox.send(self.msg);
          });
        } else {
          // fetch the soft session limit from group, and then start the timer
          if ( this.group && this.group.id !== '' && this.group.softSessionLimit !== 0 ) {
            this.sessionTimer.startTimer(this.group.softSessionLimit);
          }

          this.delegate.send(msg);
        }
      },
      javaCode: `Object object = msg.getObject();
if ( object instanceof RPCErrorMessage && ((RPCErrorMessage) object).getData() instanceof RemoteException &&
    "foam.nanos.auth.AuthenticationException".equals(((RemoteException) ((RPCErrorMessage) object).getData()).getId()) ) {
RemoteException e = (RemoteException) ((RPCErrorMessage) object).getData();
foam.nanos.logger.Logger logger = (foam.nanos.logger.Logger) getX().get("logger");
logger.warning(this.getClass().getSimpleName(), "send", e.getMessage());
  // TODO: should this be wrapped in new Thread() ?
  ((Runnable) getX().get("requestLogin")).run();
  getClientBox().send(getMsg());
} else if ( getDelegate() != null ) {
  getDelegate().send(msg);
}`
    }
  ]
});


foam.CLASS({
  package: 'foam.box',
  name: 'SessionClientBox',
  extends: 'foam.box.ProxyBox',

  requires: [ 'foam.box.SessionReplyBox' ],

  imports: [
    'sessionID as jsSessionID'
  ],

  constants: [
    {
      name: 'SESSION_KEY',
      value: 'sessionId',
      type: 'String'
    }
  ],

  properties: [
    {
      class: 'String',
      name: 'sessionID',
      factory: function() { return this.jsSessionID; }
    }
  ],

  methods: [
    {
      name: 'send',
      code: function send(msg) {
        msg.attributes[this.SESSION_KEY] = this.sessionID;

        msg.attributes.replyBox.localBox = this.SessionReplyBox.create({
          msg:       msg,
          clientBox: this,
          delegate:  msg.attributes.replyBox.localBox
        });

        this.delegate.send(msg);
      },
      swiftCode: `
let msg = msg!
msg.attributes[foam_box_SessionClientBox.SESSION_KEY] = sessionID
msg.attributes["replyBox"] = SessionReplyBox_create([
  "msg": msg,
  "clientBox": self,
  "delegate": msg.attributes["replyBox"] as? foam_box_Box,
])
try delegate.send(msg)
      `,
      javaCode: `
msg.getAttributes().put(SESSION_KEY, getSessionID());
getDelegate().send(msg);`
    }
  ]
});
