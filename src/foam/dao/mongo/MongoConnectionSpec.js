/**
 * @license
 * Copyright 2023 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.dao.mongo',
  name: 'MongoConnectionSpec',

  javaImports: [
    'java.util.Map'
  ],

  properties: [
    {
      class: 'String',
      name: 'protocol',
      value: 'mongodb'
    },
    {
      class: 'String',
      name: 'username'
    },
    {
      class: 'String',
      name: 'password'
    },
    {
      class: 'String',
      name: 'host'
    },
    {
      class: 'String',
      name: 'database'
    },
    {
      class: 'Map',
      name: 'specs'
    }
  ],

  methods: [
    {
      name: 'buildConnectionURI',
      type: 'String',
      javaCode: `
        String uri = getProtocol() + "://"
                      + getUsername() + ":"
                      + getPassword() + "@"
                      + getHost() + "/"
                      + getDatabase();
        if ( getSpecs() != null && getSpecs().size() > 0 ) {
          uri += "?";
          Map<String, String> specs = getSpecs();
          for ( Map.Entry<String, String> entry : specs.entrySet() ) {
            uri += entry.getKey() + "=" + entry.getValue() + "&";
          }
        }
        return uri;
      `
    }
  ]
})