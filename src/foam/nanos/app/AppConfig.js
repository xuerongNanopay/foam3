/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.nanos.app',
  name: 'AppConfig',

  javaImports: [
    'foam.nanos.theme.Theme',
    'foam.nanos.theme.Themes',
    'org.eclipse.jetty.server.Request'
  ],

  properties: [
    {
      class: 'String',
      name: 'name'
    },
    {
      class: 'String',
      name: 'pom',
      value: 'pom',
      documentation: 'Name of POM file to provide to foam.js loader (minus .js extension).'
    },
    {
      class: 'String',
      name: 'foamUrl',
      value: '../../../../foam3/src/foam.js',
      documentation: 'Location of FOAM for development build'
    },
    {
      documentation: 'Set at startup in bootscript.',
      class: 'String',
      name: 'version',
      javaSetter: `
      // Explicitly set in bootscript from jar manifest or services.0.
      // Ignore updates from runtime journals.  If updates are not
      // supressed, then on next upgrade the VirtualHostRoutingServlet
      // will craft an index.html with references to a, now, non-existant
      // foam-bin-x.y.z.js file.
      synchronized ( this ) {
        version_ = foam.nanos.app.AppConfig.class.getPackage().getImplementationVersion();
        if ( foam.util.SafetyUtil.isEmpty(version_) ) {
          version_ = val;
        }
        versionIsSet_ = true;
      }
      `
    },
    {
      class: 'String',
      name: 'privacy',
      value: 'Privacy Policy'
    },
    {
      class: 'String',
      name: 'privacyUrl'
    },
    {
      class: 'String',
      name: 'copyright'
    },
    {
      documentation: 'Set by Theme',
      class: 'String',
      name: 'url',
      value: 'http://localhost:8080'
    },
    {
      class: 'String',
      name: 'urlLabel',
      value: 'FOAM Powered'
    },
    {
      class: 'String',
      name: 'termsAndCondLabel',
      value: 'Terms and Conditions'
    },
    {
      class: 'String',
      name: 'termsAndCondLink'
    },
    {
      class: 'Enum',
      of: 'foam.nanos.app.Mode',
      name: 'mode'
    },
    {
      class: 'String',
      name: 'appLink',
      documentation: 'Link to Apple App Store for the app',
      visibility: 'HIDDEN'
    },
    {
      class: 'String',
      documentation: 'Link to Google play store for the app, used in LoginView. Configure in themes.jrl for each app per theme.',
      name: 'playLink',
      visibility: 'HIDDEN'
    },
    {
      class: 'String',
      name: 'externalUrl'
    }
  ],

  methods: [
    {
      name: 'configure',
      args: [
        {
          name: 'x',
          type: 'Context'
        },
        {
          name: 'url',
          type: 'String'
        }
      ],
      type: 'foam.nanos.app.AppConfig',
      javaCode: `
      return this;
      `
    }
  ]
});
