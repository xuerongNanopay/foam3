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
    'foam.dao.DAO',
    'static foam.mlang.MLang.EQ',
    'foam.nanos.geocode.model.*',
    'foam.nanos.geocode.configure.GooglePlaceServiceConfigure',
    'com.google.maps.places.v1.AutocompletePlacesRequest',
    'com.google.maps.places.v1.AutocompletePlacesResponse',
    'com.google.maps.places.v1.PlacesClient',
  ],

  methods: [
    {
      name: 'getConfigure',
      args: 'Context x',
      type: 'GooglePlaceServiceConfigure',
      javaCode: `
        var spid = x.get("spid");
        var configureDAO = (DAO) x.get("googlePlaceServiceConfigureDAO");
        var configure = (GooglePlaceServiceConfigure) configureDAO.find(EQ(GooglePlaceServiceConfigure.SPID, spid));
        if ( configure == null ) {
          throw new RuntimeException("GooglePlaceService don't find configure with spid \`" + spid + "\`");
        }
        return configure;
      `
    },
    {
      name: 'placeAutocomplete',
      args: 'Context x, PlaceAutocompleteReq req',
      type: 'PlaceAutocomplete',
      javaCode: `

        try (PlacesClient placesClient = PlacesClient.create()) {
          // AutocompletePlacesRequest request =
          //   AutocompletePlacesRequest.newBuilder()
          //   .setInput("input100358090")

        } catch ( Exception e ) {

        }

        return null;
      `
    }
  ]
})