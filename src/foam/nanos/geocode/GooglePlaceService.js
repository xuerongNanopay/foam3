/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.nanos.geocode',
  name: 'GooglePlaceService',
  documentation: `
    Implement PlaceService using Google Place Service API.
  `,

  implements: [
    'foam.nanos.geocode.PlaceService'
  ],

  javaImports: [
    'foam.nanos.geocode.model.*'
  ],

  methods: [
    {
      name: 'placeAutocomplete',
      args: 'Context x, PlaceAutocompleteReq req',
      type: 'PlaceAutocomplete',
      javaCode: `
        return null;
      `
    }
  ]
})