/**
 * @license
 * Copyright 2016 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.core',
  name: 'Int',
  extends: 'Property',

  properties: [
    'units',
    [ 'value', 0 ],
    'min',
    'max',
    [ 'type', 'Integer' ],
    [ 'adapt', function adaptInt(_, v) {
      return typeof v === 'number' ? Math.trunc(v) :
        v ? parseInt(v) :
        0 ;
      }
    ],
    [ 'fromString', function intFromString(str) {
        return str ? parseInt(str) : 0;
      }
    ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'String',
  extends: 'Property',

  documentation: 'StringProperties coerce their arguments into Strings.',

  properties: [
    {
      class: 'Boolean',
      name: 'trim',
      value: false
    },
    { class: 'Int', name: 'width', value: 30 },
    {
      name: 'adapt',
      value: function(_, a, p) {
        if ( foam.Object.isInstance(a) ) {
          if ( a[foam.locale] !== undefined )
            return a[foam.locale];
          if ( a[foam.locale.substring(0, foam.locale.indexOf('-'))] !== undefined )
            return a[foam.locale.substring(0, foam.locale.indexOf('-'))];
          return a['en'];// default language.
        }
        var s = typeof a === 'function' ||
                typeof a === 'number'   ? String(a)                :
                a && a.toString         ? a.toString()             :
                                          ''                       ;
        return s;
      }
    },
    [ 'normalize', function(value, p) { return p.trim ? value.trim() : value; } ],
    [ 'type', 'String' ],
    [ 'value', '' ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'I18NString',
  extends: 'String',

  documentation: 'A String which needs to be internationalized before being displayed to users.',

  properties: [
   {
     name: 'getter_',
     value: function(proto, prop, obj, key) {
       if ( foam.core.I18NString.GETTER__ ) return foam.core.I18NString.GETTER__(proto, prop, obj, key);
       var msg_ = obj.instance_[key];
       if ( ! foam.i18n || ! foam.xmsg ) return msg_;
       return foam.i18n.Lib.createText(prop.sourceCls_.id + '.' + this.name, msg_);
      }
   },
   {
     name: 'expression',
     preSet: function(o, n) {
       var prop = this;
       var name = this.name;
       if ( ! foam.i18n || ! foam.xmsg ) return n;
       n.apply = function(o, a) {
         var ret = n.call(o, a[0], a[1], a[2], a[3], a[4], a[5], a[6]);
         if ( ! foam.i18n || ! foam.xmsg ) return ret;
         return foam.i18n.Lib.createText(prop.sourceCls_.id + '.' + name, ret, ret);
       };
       return n;
     }
     /*
     value: function(o, n, prop) {
       if ( ! foam.i18n || ! foam.xmsg || ! prop.sourceCls_ ) return n;
       return foam.i18n.Lib.createText(prop.sourceCls_.id + '.' + prop.name, n);
     }
     */
   }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'FormattedString',
  extends: 'String',
  documentation: 'A delimiter separated string of digits',

  properties: [
    {
      class: 'FObjectProperty',
      of: 'foam.u2.TextFormatter',
      name:'formatter',
      value: null,
      documentation: `
        A TextFormatter Object to be passed to the FormattedTextField view.
      `
    }
  ],

  methods: [
    // create an extra property: formatted${propname} used to access
    // a formatted version of this string
    function installInClass(cls) {
      this.SUPER(cls);
      var capitalized = foam.String.capitalize(this.name);
      var constantize = foam.String.constantize(this.name);
      var prop = foam.core.String.create({
        forClass_: cls.id,
        sourceCls_: cls,
        name: 'formatted' + capitalized,
        hidden: true,
        javaSetter: ``,
        javaGetter: `return Formatted${capitalized}Factory_();`,
        javaFactory: `
          try {
            java.lang.reflect.Method method = ${cls.name}.${constantize}.getClass().getMethod("getFormatted", Object.class);
            this.formatted${capitalized}_ = (String) method.invoke(${cls.name}.${constantize}, (Object) this);
            this.formatted${capitalized}IsSet_ = true;
            return this.formatted${capitalized}_;
          }
          catch (NoSuchMethodException e) { }
          catch (IllegalAccessException e) { }
          catch (java.lang.reflect.InvocationTargetException e) { }
          return null;
        `
      });
      cls.axiomMap_[prop.name] = prop;
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'ModelDocumentationRefinement',
  refines: 'foam.core.Model',

  documentation: 'Upgrade Mode.documentation to a proper String property.',

  properties: [
    { class: 'String', name: 'documentation' }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Date',
  extends: 'Property',

  // documentation: 'Describes properties of type Date.',
  label: 'Date',

  properties: [
    {
      name: 'toJSON',
      value: function toJSON(value, outputter) {
        // A Date property can be transmitted as a plain timestamp.
        // Since we know the type information we will adapt a timestamp
        // back to a Date.
        return value == null ? null :
          outputter.formatDatesAsNumbers ?
          value.getTime() :
          value.toISOString();
      }
    },
    {
      name: 'adapt',
      value: function (_, d) {
        if ( typeof d === 'number' ) d = new Date(d);
        if ( typeof d === 'string' ) {
          var ret = new Date(d);

          if ( isNaN(ret.getTime()) ) {
            ret = foam.Date.MAX_DATE;
            console.warn("Invalid date: " + d + "; assuming " + ret.toISOString() + ".");
            return ret;
          }

          d = ret;
        }
        if ( d == foam.Date.MAX_DATE || d == foam.Date.MIN_DATE ) return d;
        if ( foam.Date.isInstance(d) ) {
          // Convert the Date to Noon time in GMT
          const DAY = 1000*60*60*24;
          // Add many days to time so not to break for negative times before EPOCH of 1970
          var timeOfDay = (d.getTime() + 100000 * DAY) % DAY;
          return new Date(d.getTime() - timeOfDay + 12 * 60 * 60000);
        }
        return d;
      }
    },
    [ 'type', 'Date' ],
    {
      name: 'comparePropertyValues',
      value: function(o1, o2) {
        if ( ! o1 ) return o2 ? -1 : 0;
        if ( ! o2 ) return 1;

        return foam.Date.compare(o1, o2);
      }
    },
    {
      name: 'format',
      value: function(val) {
        return foam.Date.formatDate(val);
      }
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'DateTime',
  extends: 'Date',

  documentation: 'Describes properties of type DateTime.',
  label: 'Date and time',

  properties: [
    [ 'type', 'DateTime' ],
    {
      name: 'adapt',
      value: function (_, d) {
        if ( typeof d === 'number' ) return new Date(d);
        if ( typeof d === 'string' ) {
          var ret = new Date(d);

          if ( isNaN(ret.getTime()) ) {
            ret = foam.Date.MAX_DATE;
            console.warn("Invalid date: " + d + "; assuming " + ret.toISOString() + ".");
          }

          return ret;
        }
        return d;
      }
    },
    {
      name: 'format',
      value: function(val, timeFirst = false) {
        return foam.Date.formatDate(val, timeFirst);
      }
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Time',
  extends: 'String',

  documentation: 'Describes properties of type Time.',
  label: 'Time',

  properties: [
    [ 'type', 'time' ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Byte',
  extends: 'Int',

  documentation: 'Describes properties of type Byte.',
  label: 'Round byte numbers',

  properties: [
    [ 'type', 'Byte' ],
    [ 'min', -128 ],
    [ 'max', 127 ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Short',
  extends: 'Int',

  documentation: 'Describes properties of type Short.',
  label: 'Round short numbers',

  properties: [
    [ 'type', 'Short' ],
    [ 'min', -32768 ],
    [ 'max', 32767 ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name:  'Long',
  extends: 'Int',

  documentation:  'Describes properties of type Long.',
  label: 'Round long numbers',

  properties: [
    [ 'type', 'Long' ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Float',
  extends: 'Int',

  // documentation:  'Describes properties of type Float.',
  label: 'Decimal numbers',

  properties: [
    'precision',
    [
      'adapt',
      function (_, v) {
        return typeof v === 'number' ? v : v ? parseFloat(v) : 0.0 ;
      }
    ],
    [ 'type', 'Float' ]
  ]
});


/**
 No different than Float for JS, but useful when targeting with other languages.
 **/
foam.CLASS({
  package: 'foam.core',
  name: 'Double',
  extends: 'Float',
  properties: [
    [ 'type', 'Double' ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Function',
  extends: 'Property',

  documentation: 'Describes properties of type Function.',
  label: 'Code that can be run',

  properties: [
    [
      'value',
      function() {}
    ],
    [
      'adapt',
      function(o, n, prop) {
        // if boolean, return a function that returns the same boolean
        // Useful for overriding functions with no-op in jrls and JSON
        if ( ( foam.Undefined.isInstance(n) || foam.Null.isInstance(n) ) && foam.Boolean.isInstance(n) ) { return () => n }
        return n;
      }
    ],
    [
      'assertValue',
      function(value, prop) {
        foam.assert(typeof value === 'function', prop.name, 'Cannot set to non function type.');
      }
    ]
  ]
});



foam.CLASS({
  package: 'foam.core',
  name: 'Object',
  extends: 'Property',
  documentation: '',
  properties: [
    [ 'type', 'Any' ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Array',
  extends: 'Property',

  properties: [
    [
      'factory',
      function() { return []; }
    ],
    [
      'isDefaultValue',
      function(v) { return ! v || ! v.length; }
    ],
    [ 'type', 'Any[]' ]
  ],

  methods: [
    function installInProto(proto) {
      this.SUPER(proto);
      var self = this;
      ['push','splice','unshift'].forEach(func => {
        Object.defineProperty(proto, self.name + '$' + func, {
          get: function classGetter() {
            return function (...args) {
              // Push value
              let val = this[self.name][func](...args);
              // Force property update
              this.propertyChange.pub(self.name, this.slot(self.name));
              return val;
            }
          },
          configurable: true
        });
      })
      Object.defineProperty(proto, self.name + '$remove', {
        get: function classGetter() {
          return function (predicate) {
            // Faster than splice or filter as of the time this was added
            let oldArry = this[self.name];
            let newArry = [];
            for ( let i=0 ; i < oldArry.length ; i++ ) {
              if ( ! predicate.f(oldArry[i]) ) {
                newArry.push(oldArry[i]);
              }
            }
            this[self.name] = newArry;
          }
        },
        configurable: true
      });
      Object.defineProperty(proto, self.name + '$replace', {
        get: function classGetter() {
          return function (predicate, value) {
            // Faster than splice or filter as of the time this was added
            let arry = this[self.name];
            for ( let i=0 ; i < arry.length ; i++ ) {
              if ( predicate.f(arry[i]) ) {
                arry[i] = value;
              }
            }
            // Force property update
            this.propertyChange.pub(self.name, this.slot(self.name));
          }
        },
        configurable: true
      });
      // Does not modify the original array; returns an array containing all
      //   elements that satisfy the provided predicate.
      Object.defineProperty(proto, self.name + '$filter', {
        get: function classGetter() {
          return function (predicate) {
            return foam.Array.filter(this[self.name], predicate);
          }
        },
        configurable: true
      });
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'List',
  extends: 'foam.core.Object',
  properties: [
    [ 'type', 'List' ],
    [
      'factory',
      function() { return []; }
    ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'StringArray',
  extends: 'Property',

  documentation: 'An array of String values.',
  label: 'List of text strings',

  properties: [
    {
      name: 'of',
      value: 'String',
      documentation: 'The FOAM sub-type of this property.'
    },
    [ 'type', 'String[]' ],
    [
      'factory',
      function() { return []; }
    ],
    [
      'adapt',
      function(_, v, prop) {
        if ( ! Array.isArray(v) ) return v;

        var copy;
        for ( var i = 0 ; i < v.length ; i++ ) {
          if ( typeof v[i] !== 'string' ) {
            if ( ! copy ) copy = v.slice();
            copy[i] = prop.adaptArrayElement.call(this, v[i], prop);
          }
        }

        return copy || v;
      }
    ],
    [
      'adaptArrayElement',
      function(o, prop) {
        return String(o);
      }
    ],
    [
      'assertValue',
      function(v, prop) {
        if ( v === null ) return;

        foam.assert(Array.isArray(v),
          prop.name, 'Tried to set StringArray to non-array type.');
        for ( var i = 0 ; i < v.length ; i++ ) {
          foam.assert(
            typeof v[i] === 'string',
            prop.name, 'Element', i, 'is not a string', v[i]);
        }
      }
    ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'IntegerArray',
  extends: 'Property',

  // Not named IntArray because 'Array' is faceted so a class: 'Array', of 'Int' will be broken.
  documentation: 'An array of Int values.',

  label: 'List of integers',

  properties: [
    {
      name: 'of',
      value: 'Int'
    },
    [ 'type', 'int[]' ],
    [
      'factory',
      function() { return []; }
    ],
    [
      'adapt',
      function(_, v, prop) {
        if ( v == '' ) return [];
        if ( foam.String.isInstance(v) ) v = v.split(',');

        if ( ! Array.isArray(v) ) return [];

        var copy;
        for ( var i = 0 ; i < v.length ; i++ ) {
          if ( typeof v[i] !== 'number' ) {
            if ( ! copy ) copy = v.slice();
            copy[i] = prop.adaptArrayElement.call(this, v[i], prop);
          }
        }

        return copy || v;
      }
    ],
    [
      'adaptArrayElement',
      function(o, prop) {
        return (o).valueOf();
      }
    ],
    [
      'assertValue',
      function(v, prop) {
        if ( v === null ) return;

        foam.assert(Array.isArray(v),
          prop.name, 'Tried to set IntArray to non-array type.');
        for ( var i = 0 ; i < v.length ; i++ ) {
          foam.assert(
            typeof v[i] === 'number',
            prop.name, 'Element', i, 'is not a number', v[i]);
        }
      }
    ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Class',
  extends: 'Property',

  properties: [
    {
      name: 'toJSON',
      value: function toJSON(value, _) {
        return value && value.id;
      }
    },
    [
      'adapt',
      function(_, v) {
        if ( v && v.class === '__Class__' )
          return v.forClass_;
        return v;
      }
    ],
    [ 'type', 'Class' ],
    [ 'displayWidth', 80 ],
    [ 'cloneProperty', function(value, cloneMap, _, obj) {
        cloneMap[this.name] = obj.instance_[this.name];
      }
    ]
  ],

  methods: [
    function installInProto(proto) {
      this.SUPER(proto);

      // Wrap the getter that was installed with an adapter that will perform the lookup.
      // We don't adapt at set time because the class were referring to might not be loaded
      // at that point.
      var name = this.name;
      var desc = Object.getOwnPropertyDescriptor(proto, name);

      var adapt = function(value) {
        if ( foam.String.isInstance(value) ) {
          var cls = this.__context__.maybeLookup(value);
          if ( ! cls ) { // if the model is not available, it will be set on each get()
            console.error(`Property '${name}' of type '${this.model_.name}' was set to '${value}', which isn't a valid class (yet).`);
            return null;
          }
          return cls;
        }
        return value;
      };

      var get = desc.get;
      desc.get = function() { return adapt.call(this, get.call(this)); };

      Object.defineProperty(proto, name, desc);
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'EMail',
  extends: 'String',
  // FUTURE: verify
  label: 'Email address',
  properties: [
    [ 'displayWidth', 50 ],
    [ 'trim', true ],
    [
      'preSet',
      function(_, v) {
        return v.toLowerCase();
      }
    ]
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Image',
  extends: 'String',
  // FUTURE: verify
  label: 'Image data or link',
  properties: [ [ 'displayWidth', 80 ] ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'URL',
  extends: 'String',
  // FUTURE: verify
  label: 'Web link (URL or internet address)',
  properties: [ [ 'displayWidth', 80 ] ]
});

foam.CLASS({
  package: 'foam.core',
  name: 'InternalLink',
  extends: 'URL',
  label: 'Link to nano service (eg. /service/serviceA) or menu (eg. #menu_1) in the app.',
  help: 'Do not inclulde domain name in the link as it will be resolved on the client.',
  properties: [ [ 'displayWidth', 80 ] ],
  methods: [
    function installInProto(proto) {
      this.SUPER(proto);
      var self = this;
      Object.defineProperty(proto, self.name + '$completeURL', {
        get: function completeURL() {
          return this.__context__.window.location.origin + this[self.name];
        },
        configurable: true
      });
    }
  ]
});

foam.CLASS({
  package: 'foam.core',
  name: 'Website',
  extends: 'URL',
  label: `Websites (requires 'http(s)'/'www' links)`
});


foam.CLASS({
  package: 'foam.core',
  name: 'Color',
  extends: 'String',
  label: 'Color',
  properties: [ [ 'displayWidth', 20 ] ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Password',
  extends: 'String',
  label: 'Password that displays protected or hidden text'
});


foam.CLASS({
  package: 'foam.core',
  name: 'PhoneNumber',
  extends: 'String',
  label: 'Phone number',
  properties: [ [ 'displayWidth', 20 ] ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Code',
  extends: 'String'
});


foam.CLASS({
  package: 'foam.core',
  name: 'UnitValue',
  extends: 'Long',
  properties: [
    {
      class: 'String',
      name: 'unitPropName',
      documentation: `
        The name of the property of a model that contains the denomination String.
      `
    },
    {
      name: 'unitPropValueToString',
      value: async function(x, val, unitPropName, excludeUnit) {
        if ( unitPropName ) {
          const unitProp = await x.currencyDAO.find(unitPropName);
          if ( unitProp )
            return unitProp.format(val, excludeUnit, false);
        }
        return val;
      }
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Map',
  extends: 'Property',

  // TODO: Remove need for sorting
  properties: [
    [ 'factory', function() { return {} } ],
    [
      'comparePropertyValues',
      function(o1, o2) {
        if ( foam.typeOf(o1) != foam.typeOf(o2) ) return -1;

        var keys1 = Object.keys(o1).sort();
        var keys2 = Object.keys(o2).sort();
        if ( keys1.length < keys2.length ) return -1;
        if ( keys1.length > keys2.length ) return 1;
        for ( var i = 0 ; i < keys1.length ; i++ ) {
          var c = foam.String.compare(keys1[i], keys2[i]);
          if ( c != 0 ) return c;
          c = foam.util.compare(o1[keys1[i]], o2[keys2[i]]);
          if ( c != 0 ) return c;
        }

        return 0;
      }
    ],
    [
      'cloneProperty',
      function(value, cloneMap) {
        if ( value ) {
          var tmp = cloneMap[this.name] = {};
          for ( var key in value ) {
            tmp[key] = value[key];
          }
        }
      }
    ],
    [
      'diffPropertyValues',
      function(o1, o2) {
        // TODO
      }
    ],
    [ 'type', 'Map' ]
  ],

  methods: [
    function installInProto(proto) {
      this.SUPER(proto);
      var self = this;
      Object.defineProperty(proto, self.name + '$set', {
        get: function mapSet() {
          return function (k, v) {
            // Set value on map
            this[self.name][k] = v;
            // Force property update
            this.propertyChange.pub(self.name, this.slot(self.name));
          }
        },
        configurable: true
      });
      Object.defineProperty(proto, self.name + '$remove', {
        get: function mapRemove() {
          return function (k) {
            // Remove value from map
            delete this[self.name][k];
            // Force property update
            this.propertyChange.pub(self.name, this.slot(self.name));
          }
        },
        configurable: true
      })
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'FObjectProperty',
  extends: 'Property',

  properties: [
    {
      class: 'Class',
      name: 'of',
      value: 'foam.core.FObject'
    },
    {
      name: 'type',
      factory: function() {
        return this.of.id;
      }
    },
    {
      name: 'fromJSON',
      value: function(json, ctx, prop) {
        return foam.json.parse(json, foam.lookup(prop.type), ctx);
      }
    },
    {
      name: 'adapt',
      value: function(_, v, prop) {
        // All FObjects may be null.
        if ( v === null ) return v;

        var type = foam.lookup(prop.type);

        // Example: type = Predicate and v=foam.mlang.predicate.True
        if ( type.isSubClass(v) ) {
          console.warn('Invalid setting of property to class rather than instance for ', prop.name, 'of type', type.id);
          return v.create();
        }

        return type.isInstance(v) ?
          v :
          ( v.class ?
            this.__context__.lookup(v.class) :
            type ).create(v, this.__subContext__);
      }
    },
    {
      name: 'cloneProperty',
      value: function(value, cloneMap, opt_X) {
        cloneMap[this.name] = value && value.clone ? value.clone(opt_X) : value;
      }
    },
    // Override copyFrom behaviour
    ['copyValueFrom', function copyValueFrom(targetObj, sourceObj) {
        var name = this.name;
        if ( targetObj[name] && sourceObj[name] ) {
          targetObj[name].copyFrom(sourceObj[name]);
          return true;
        }
        return false;
      }
    ],
  ],

  methods: [
    function xinitObject(obj) {
      var s1, s2;

      obj.onDetach(function() {
        s1 && s1.detach();
        s2 && s2.detach();
      });

      var name = this.name;
      var slot = this.toSlot(obj);

      function proxyListener(sub) {
        var args = [
          'nestedPropertyChange', name, slot
        ].concat(Array.from(arguments).slice(1));

        obj.pub.apply(obj, args);
      }

      function attach(inner) {
        s1 && s1.detach();
        s1 = inner && inner.sub && inner.sub('propertyChange', proxyListener);

        s2 && s2.detach();
        s2 = inner && inner.sub && inner.sub('nestedPropertyChange', proxyListener);
      }

      function listener(s, pc, name, slot) {
        attach(slot.get());
      }

      obj.sub('propertyChange', name, listener);

      // TODO: Only hook up the subscription when somebody listens to us.
      if ( obj[name] ) attach(obj[name]);
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'Reference',
  extends: 'Property',

  properties: [
    {
      class: 'Class',
      name: 'of'
    },
    [ 'type', 'Any' ],
    {
      class: 'String',
      name: 'targetDAOKey',
      expression: function(of) {
        if ( ! of ) {
          console.error("invalid 'of' for property with targetDAOKey", this.name);
        }
        return foam.String.daoize(of.name);
      }
    },
    {
      class: 'Boolean',
      name: 'enableLink',
      documentation: `
        Create the reference view as an anchor link to the reference's DetailView or provided menu.
        Check ReadReferenceView documentation for more info.
      `,
      value: true
    },
    {
      class: 'Boolean',
      name: 'showSubColumns',
      documentation: 'Allow for selection of referenced columns in table views.',
      value: true
    },
    {
      name: 'menuKeys',
      documentation: `
        A list of menu ids.
        The link will reference to the first menu to which group has permission
        in this list. If no menus are permissioned, the link will be disabled.
        Check ReadReferenceView documentation for more info.
      `
    },
    {
      class: 'String',
      name: 'unauthorizedTargetDAOKey',
      documentation: `
        Can be provided to use unauthorized local DAOs when the context user is the SYSTEM USER.
      `
    },
    {
      name: 'adapt',
      value: function(oldValue, newValue, prop) {
        return prop.of.isInstance(newValue) ?
          newValue.id :
          newValue ;
      }
    },
    {
      name: 'value',
      expression: function(of) {
        if ( of && ! of.ID ) {
          console.warn('of.ID not found for: ' + of + '.' +this.name);
        }
        var ret = of ? of.ID.value : null;

        if ( ! of ) {
          console.warn('Of not found for: ' + this.name);
          console.warn('Possible circular reference: Please explicitly set a default value on: ' + this.name);
        }

        if ( ret === undefined ) {
          console.warn('Default value is undefined for: ' + of.name + '.' + this.name);
          ret = null;
        }

        return ret;
        // return ( of && of.ID.value ) || null;
      }
    }
  ],

  methods: [
    function installInProto(proto) {
      this.SUPER(proto);
      var self    = this;
      var daoName = self.name + '$dao';

      Object.defineProperty(proto, daoName, {
        get: function classGetter() {
          var dao = this.__subContext__[self.targetDAOKey] || this[self.targetDAOKey];
          if ( ! dao )
            console.warn(`Missing Reference DAO: ${self.targetDAOKey} for property ${proto.cls_.id}.${self.name}`);
          return dao;
        },
        configurable: true
      });

      Object.defineProperty(proto, self.name + '$find', {
        get: function classGetter() {
          return this[daoName].find(this[self.name]);
        },
        configurable: true
      });
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'PropertyShortNameRefinement',
  refines: 'Property',

  properties: [
    /**
      A short-name is an optional shorter name for a property.
      It is used by JSON and XML support when 'useShortNames'
      is enabled. Short-names enable output to be smaller,
      which can save disk space and/or network bandwidth.
      Ex.
    <pre>
      properties: [
        { name: 'firstName', shortName: 'fn' },
        { name: 'lastName',  shortName: 'ln' }
      ]
    </pre>
    */
    { class: 'String', name: 'name', required: true },
    {
      class: 'I18NString',
      name: 'label',
      expression: function(name) { return foam.String.labelize(name); }
    },
    {
      name: 'labelFormatter',
      value: function(_, prop) {
        this.add(prop.label$);
      }
    },
    { class: 'String', name: 'shortName' }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'ModelUpgradeTypesRefinement',
  refines: 'foam.core.Model',

  documentation: 'Update Model Property types.',

  properties: [
    { class: 'String',  name: 'name' },
    {
      class: 'I18NString',
      name: 'label',
      expression: function(name) { return foam.String.labelize(name); }
    },
    { class: 'Boolean', name: 'abstract' }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'FacetedPropertyRefinement',
  refines: 'foam.core.Property',

  axioms: [
    foam.pattern.Faceted.create()
  ],

  properties: [
    {
      name: 'of'
    }
  ]
});


// Upgrade async property to a real boolean property.
foam.CLASS({
  package: 'foam.core',
  name: 'AbstractMethodUpgradeTypesRefinement',
  refines: 'foam.core.AbstractMethod',
  properties: [
    {
      class: 'Boolean',
      name: 'async',
      value: false
    }
  ]
});


// TODO: When value:'s get adapt:'ed, then we should cleanup all instances of this.
foam.CLASS({
  package: 'foam.core',
  name: 'GlyphProperty',
  extends: 'FObjectProperty',

  requires: [ 'foam.core.Glyph' ],

  properties: [
    [ 'value', null ],
    {
      name: 'adapt',
      value: function(_, v, prop) {
        if ( ! v ) return;
        if ( foam.String.isInstance(v) ) {
          return prop.Glyph.create({ themeName: v });
        }
        if ( ! foam.core.FObject.isInstance(v) ) {
          return prop.Glyph.create(v);
        }
        return v;
      }
    }
  ]
});


foam.CLASS({
  package: 'foam.core',
  name: 'FUIDProperty',
  extends: 'Property',

  properties: [
    {
      name: 'adapt',
      value: function(_, a) {
        return a ? a.toString().trim() : '';
      }
    },
    [ 'type', 'String' ],
    [ 'value', '' ]
  ]
});
