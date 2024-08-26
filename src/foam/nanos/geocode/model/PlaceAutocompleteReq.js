/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.INTERFACE({
  package: 'foam.nanos.geocode.model',
  name: 'PlaceAutocompleteReq',
  documentation: `
    Model that using by 'placeAutocomplete' method in PlaceService 
  `,

  properties: [
    {
      class: 'String',
      name: 'region',
      documentation: `
        The region code, specified as a ccTLD ("top-level domain") two-character value.
        see: https://en.wikipedia.org/wiki/Country_code_top-level_domain
      `
    }
  ]
})