/**
* @license
* Copyright 2023 The FOAM Authors. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/

foam.CLASS({
  package: 'foam.nanos.app',
  name: 'LegalMenu',
  extends: 'foam.nanos.menu.PseudoMenu',

  imports: [
    'appConfig',
    'window'
  ],

  requires: [
    'foam.nanos.menu.Menu'
  ],

  messages: [
    { name: 'TERMS_AND_CONDITIONS_TITLE', message: 'Terms and Conditions' },
    { name: 'PRIVACY_TITLE', message: 'Privacy Policy' }
  ],

  properties: [
    {
      name: 'children_',
      factory: function() {
        var aDAO = this.MDAO.create({of: this.Menu});
        if ( this.appConfig.termsAndCondLink ) {
          aDAO.put(this.Menu.create({
            id: this.id + '/TC',
            parent: this.id,
            label: this.TERMS_AND_CONDITIONS_TITLE,
            handler: {
              class: 'foam.nanos.menu.ViewMenu',
              shouldResetBreadcrumbs: false,
              view: {
                class: 'foam.u2.IFrameDocView',
                src$: this.slot(function(appConfig$termsAndCondLink) {
                  if ( appConfig$termsAndCondLink.startsWith("http") )
                    return appConfig$termsAndCondLink;
                  return this.window.location.origin + appConfig$termsAndCondLink;
                })
              }
            }
          }));
        }
        if ( this.appConfig.privacyUrl ) {
          aDAO.put(this.Menu.create({
            id: this.id + '/privacy',
            parent: this.id,
            label: this.PRIVACY_TITLE,
            handler: {
              class: 'foam.nanos.menu.ViewMenu',
              shouldResetBreadcrumbs: false,
              view: {
                class: 'foam.u2.IFrameDocView',
                src$: this.slot(function(appConfig$privacyUrl) {
                  if ( appConfig$privacyUrl.startsWith("http") )
                    return appConfig$privacyUrl;
                  return this.window.location.origin + appConfig$privacyUrl;
                })
              }
            }
          }));
        }
        return aDAO;
      }
    }
  ]
});
