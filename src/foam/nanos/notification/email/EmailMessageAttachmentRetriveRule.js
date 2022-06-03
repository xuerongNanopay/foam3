/**
* @license
* Copyright 2022 The FOAM Authors. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

foam.CLASS({
  package: 'foam.nanos.notification.email',
  name: 'EmailMessageAttachmentRetriveRule',
  extends: 'foam.nanos.ruler.Rule',
  documentation: 'After Rule for retrieving attachments from email and put into fileDAO',
  
  properties: [
    {
      name: 'action',
      transient: true,
      visibility: 'HIDDEN',
      javaGetter: 'return new EmailMessageAttachmentRetriveRuleAction(getX());'
    }
  ]
});