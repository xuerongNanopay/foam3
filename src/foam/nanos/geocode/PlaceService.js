/**
 * @license
 * Copyright 2024 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.INTERFACE({
  package: 'foam.nanos.geocode',
  name: 'PlaceService',

  proxy: true,
  skeleton: true,
  client: true,

  methods: [
    {
      name: 'placeAutocomplete',
      args: 'Context x, foam.nanos.geocode.model.PlaceAutocompleteReq req',
      type: 'foam.nanos.geocode.model.PlaceAutocomplete',
    }
  ]
})