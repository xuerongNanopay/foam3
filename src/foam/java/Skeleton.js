/**
 * @license
 * Copyright 2016 Google Inc. All Rights Reserved.
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
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
  package: 'foam.java',
  name: 'Skeleton',

  properties: [
    {
      class: 'Class',
      name: 'of'
    },
    {
      name: 'name',
      expression: function(of) {
        return this.of.name + 'Skeleton';
      }
    },
    {
      name: 'package',
      expression: function(of) {
        return this.of.package;
      }
    },
    {
      name: 'id',
      expression: function(name, package) {
        return package + '.' + name;
      }
    }
  ],

  methods: [
    function buildJavaClass(cls) {
      cls = cls || foam.java.Class.create();

      cls.package = this.package;
      cls.name    = this.name;
      cls.extends = 'foam.box.AbstractSkeleton';

      foam.core.FObjectProperty.create({
        name: 'delegateFactory',
        type: 'foam.core.XFactory'
//        type: this.of.id
      }).buildJavaClass(cls);

      cls.method({
        type: 'void',
        visibility: 'public',
        name: 'send',
        args: [ { name: 'message', type: 'foam.box.Message' } ],
        body: this.sendMethodCode()
      });
      /*

      cls.method({
        type: 'void',
        visibility: 'public',
        name: 'setDelegateObject',
        args: 'foam.core.XFactory factory',
        body: "setDelegate((" + this.of.id + ") obj);"
      });
      */

      return cls;
    }
  ],

  templates: [
    {
      name: 'sendMethodCode',
      template: `
    if ( ! ( message.getObject() instanceof foam.box.RPCMessage) ) {
      // TODO return an error?
      return;
    }

    foam.box.RPCMessage rpc      = (foam.box.RPCMessage) message.getObject();
    foam.box.Box        replyBox = (foam.box.Box) message.getAttributes().get("replyBox");
    <%
    var methods = this.of.getOwnAxiomsByClass(foam.core.Method);
    var anyHasReturn = methods.find(function(m) { return m.javaType && m.javaType !== 'void'; });
    if ( anyHasReturn ) { %>Object result = null;<% }%>

    try {
      switch ( rpc.getName() ) {<%

  for ( var i = 0 ; i < methods.length ; i++ ) {
    var m = methods[i];
    var hasReturn = m.javaType && m.javaType !== 'void';%>
        case "<%= m.name %>":
          <% if ( hasReturn ) { %>result =<% } %> ((<%= this.of.id %>) (getDelegateFactory().create(getMessageX(message)))).<%= m.name %>(
          <%
    for ( var j = 0 ; j < m.args.length ; j++ ) {
      if ( m.args[j].type == 'Context' ) {
        %>getMessageX(message)<%
      } else if ( foam.core.AbstractEnum.isSubClass(this.__context__.maybeLookup(m.args[j].type)) ) {
        %>rpc.getArgs() != null && rpc.getArgs().length > <%= j %> ? (<%= m.args[j].javaType %>) rpc.getArgs()[<%= j %>] : null<%
      } else {
        if ( {byte: 1, double: 1, float: 1, int: 1, long: 1, short: 1 }[m.args[j].javaType] ) {
          %>to<%= m.args[j].javaType %><%
        } else {
          %>(<%= m.args[j].javaType %>)<%
        }
        %>(rpc.getArgs() != null && rpc.getArgs().length > <%= j %> ? rpc.getArgs()[<%= j %>] : null)<%
      }
      if ( j != m.args.length - 1 ) { %>,
            <% }
      }
    %>);
        break;
    <%
  }%>
        default: throw new RuntimeException("Method not found.");
      }
    } catch (Throwable t) {
      if ( t instanceof foam.core.FOAMException ) {
        RuntimeException clientE = (RuntimeException)
          ((foam.core.FOAMException) t).getClientRethrowException();
        if ( clientE != null ) {
          throw clientE;
        }
      }
      foam.nanos.logger.Loggers.logger(getMessageX(message), this).warning(((foam.nanos.boot.NSpecFactory)getDelegateFactory()).getNSpecName(), rpc.getName(), "returning exception", t.toString()); //, t);
      // NOTE: this is required for SocketClientReplyBox to find the socket that this request arrived on.  The localAttributes 'x' does not have access to the socket.
      message.setX(getX());
      message.replyWithException(t);

      return;
    }

    if ( replyBox != null ) {
      foam.box.RPCReturnMessage reply = (foam.box.RPCReturnMessage)getX().create(foam.box.RPCReturnMessage.class);
      <% if ( anyHasReturn ) { %>if ( result != null ) {
        // foam.nanos.logger.Loggers.logger(getMessageX(message), this).debug(((foam.nanos.boot.NSpecFactory)getDelegateFactory()).getNSpecName(), rpc.getName(), result);
        reply.setData(result);
      }<% } %>

      foam.box.Message message1 = getX().create(foam.box.Message.class);
      message1.setObject(reply);
      replyBox.send(message1);
    }`
    }
  ]
});
