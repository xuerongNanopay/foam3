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

  methods: [
    {
      name: 'placeAutocomplete',
      args: 'Context x, PlaceAutocompleteReq req',
      javaCode: `
      
      `
    }
  ]
})