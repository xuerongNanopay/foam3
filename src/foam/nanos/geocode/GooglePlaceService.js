/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.INTERFACE({
  package: 'foam.nanos.geocode',
  name: 'GooglePlaceService',

  implements: [
    'foam.nanos.geocode.PlaceService'
  ],

  methods: [
    {
      name: 'placeAutocomplete',
      args: 'Context x',
      javaCode: `
      
      `
    }
  ]
})