/**
* @license
* Copyright 2022 The FOAM Authors. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

foam.CLASS({
  package: 'foam.nanos.notification.email',
  name: 'EmailMessageAttachmentRetriveRuleAction',
  implements: [ 'foam.nanos.ruler.RuleAction' ],
  documentation: 'After RuleAction for retrieving attachments from email and put into fileDAO',

  javaImports: [
    'foam.dao.DAO',
    'foam.nanos.fs.File',
    'foam.nanos.logger.Loggers',
    'java.util.List',
    'java.util.ArrayList'
  ],

  methods: [
    {
      name: 'applyAction',
      javaCode: `
        EmailMessage emailMessage = (EmailMessage) obj;
        File[] files = emailMessage.getAttachmentFiles();
        DAO fileDAO = (DAO) getX().get("fileDAO");
        List<String> fileIds = new ArrayList<String>(files.length);

        for ( int i = 0 ; i < files.length ; i++ ) {
          System.out.println("$$$$ rule: " + files[i].getFilename());
          try {
            File file = (File) fileDAO.put(files[i]);
            fileIds.add(file.getId());
          } catch ( Throwable t ) {
            Loggers.logger(x, this).error("Attachment retrieving fail on file", files[i].getFilename(), "in EmailMessage.id", emailMessage.getId(), t);
          }
        }
        emailMessage.setAttachments(fileIds.toArray(new String[0]));
      `
    }
  ]
});