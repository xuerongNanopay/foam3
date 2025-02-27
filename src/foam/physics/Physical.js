/**
 * @license
 * Copyright 2014 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */


foam.CLASS({
  package: 'foam.physics',
  name: 'Physical',

  documentation: 'A Physical object has velocity and mass and may optionally be subject to friction and gravity.',

  constants: {
    INFINITE_MASS: 10000
  },

  properties: [
    { class: 'Float', name: 'friction' },
    { class: 'Float', name: 'gravity', value: 1 },
    { class: 'Float', name: 'vx', value: 0, precision: 3 },
    { class: 'Float', name: 'vy', value: 0, precision: 3 },
    {
      class: 'Float',
      name: 'velocity',
      precision: 3,
      getter: function() { return this.distance(this.vx, this.vy); },
      setter: function(v) { this.setVelocityAndAngle(v, this.angleOfVelocity); }
    },
    {
      class: 'Float',
      name: 'angleOfVelocity',
      getter: function() { return Math.atan2(this.vy, this.vx); },
      setter: function(a) { this.setVelocityAndAngle(this.velocity, a); }
    },
    { class: 'Float', name: 'mass', value: 1 }
  ],

  methods: [
    function distance(dx, dy) {
      return Math.sqrt(dx*dx + dy*dy);
    },

    function applyMomentum(m, a) {
      this.vx += (m * Math.cos(a) / this.mass);
      this.vy += (m * Math.sin(a) / this.mass);
    },

    function momentumAtAngle(a) {
      if ( this.mass === this.INFINITE_MASS ) return 0;
      var v = this.velocityAtAngle(a);
      return v * this.mass;
    },

    function velocityAtAngle(a) {
      if ( this.mass === this.INFINITE_MASS ) return 0;
      return Math.cos(a-this.angleOfVelocity) * this.velocity;
    },

    function setVelocityAndAngle(v, a) {
      this.vx = v * Math.cos(a);
      this.vy = v * Math.sin(a);

      return this;
    },

    function distanceTo(other) {
      return this.distance(this.x-other.x, this.y-other.y);
    }
  ]
});
