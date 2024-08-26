/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.nanos.geocode.configure',
  name: 'GooglePlaceServiceConfigure',

  properties: [
    {
      class: 'String',
      name: 'apiKey',
      documentation: `API_KEY for your google place service`
    }
  ]
})