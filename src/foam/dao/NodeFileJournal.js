/**
 * @license
 * Copyright 2018 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.dao',
  name: 'NodeFileJournal',
  extends: 'foam.dao.AbstractJournal',

  flags: [ 'node' ],

  properties: [
    {
      class: 'Class',
      name: 'of',
      value: 'foam.core.FObject'
    },
    {
      name: 'fd',
      required: true
    },
    {
      name: 'offset',
      factory: function () {
        var stat = this.fs.fstatSync(this.fd);
        return stat.size;
      }
    },
    {
      name: 'fs',
      factory: function () {
        return require('fs');
      }
    },
    {
      name: 'writePromise',
      value: Promise.resolve()
    }
  ],

  methods: [
    function put_(x, old, nu) {
      return this.write_(Buffer.from(
        "put(foam.json.parse(" + foam.json.Storage.stringify(nu, this.of) +
        "));\n"));
    },

    function put(x, prefix, dao, obj) {
      var old = dao.find_(x, obj.id);
      this.put_(x, old, obj);
    },

    function remove(x, prefix, dao, obj) {
      return this.write_(Buffer.from(
        "remove(foam.json.parse(" +
        foam.json.Storage.stringify(obj, this.of) +
        "));\n"));
    },

    function write_(data) {
      var self = this;
      var offset = self.offset;
      self.offset += data.length;
      return self.writePromise = self.writePromise.then(function () {
        return new Promise(function (resolve, reject) {
          self.fs.write(
            self.fd, data, 0, data.length, offset,
            function (err, written, buffer) {
              if ( err ) reject(err);
              if ( written != data.length )
                reject(new Error('foam.dao.NodeFileJournal: Incomplete write'));
              resolve();
            });
        });
      });
    },

    function replay(x, dao) {
      var self = this;
      return new Promise(function (resolve, reject) {
        self.fs.readFile(self.fd, 'utf8', function (err, data_) {
          if ( err ) {
            reject(err);
            return;
          }

          var context = {
            put: function (o) {
              return dao.put(o);
            },
            remove: function (o) {
              return dao.remove(o);
            },
            foam: {
              json: {
                parse: function (obj) {
                  return foam.json.parse(obj, self.of, dao.__context__);
                }
              }
            }
          };

          with ( context ) eval(data_);

          resolve(dao);
        });
      });
    }
  ]
});