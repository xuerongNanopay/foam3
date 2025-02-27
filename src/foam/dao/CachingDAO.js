/**
 * @license
 * Copyright 2016 Google Inc. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/**
  CachingDAO will do all queries from its fast cache. Writes
  are sent through to the src and cached before resolving any put() or
  remove().
  <p>
  You can use a foam.dao.EasyDAO with caching:true to use caching
  automatically with an indexed MDAO cache.
  <p>
  The cache maintains full copy of the src, but the src is considered the
  source of truth.
*/
foam.CLASS({
  package: 'foam.dao',
  name: 'CachingDAO',
  extends: 'foam.dao.ProxyDAO',

  requires: [
    'foam.dao.DAOSink',
    'foam.dao.PromisedDAO',
    'foam.dao.PurgeRecordCmd',
    'foam.dao.QuickSink'
  ],

  imports: [
    'document',
    'loginSuccess?',
    'setInterval'
  ],

  implements: [ 'foam.mlang.Expressions' ],

  properties: [
    {
      /** The source DAO on which to add caching. Writes go straight
        to the src, and cache is updated to match.
      */
      class: 'foam.dao.DAOProperty',
      name: 'src'
    },
    {
      /** The cache to read items quickly. Cache contains a complete
        copy of src. */
      name: 'cache'
    },
    {
      /**
        Set .cache rather than using delegate directly.
        Read operations and notifications go to the cache, waiting
        for the cache to preload the complete src state. 'Unforward'
        ProxyDAO's default forwarding of put/remove/removeAll.
        @private
      */
      class: 'Proxy',
      of: 'foam.dao.DAO',
      name: 'delegate',
      hidden: true,
      topics: [ 'on' ],
      forwards: [ 'find_', 'select_' ],
      expression: function(src) {
        var cache = this.cache;
        // The PromisedDAO resolves as our delegatec when the cache is ready to use
        return this.PromisedDAO.create({
          promise: (async function() {
            var a = await src.select();
            await cache.removeAll();
            a.array.forEach(o => cache.put(o));
            return cache;
          })()
        });
      }
    },
    {
      class: 'Int',
      name: 'pollingInterval',
      units: 'ms'
    },
    {
      class: 'FObjectProperty',
      of: 'foam.core.Property',
      name: 'pollingProperty'
    }
  ],

  methods: [
    function init() {
      this.SUPER();

      if ( this.loginSuccess$ ) {
        this.loginSuccess$.sub(this.onSrcReset);
      }

      var proxy = this.src$proxy;
      proxy.listen(this.QuickSink.create({
        putFn:    this.onSrcPut,
        removeFn: this.onSrcRemove,
        resetFn:  this.onSrcReset
      }));

      if ( this.pollingInterval > 0 ) {
        this.setInterval(this.poll, this.pollingInterval);
      }
    },

    /** Puts are sent to the cache and to the source, ensuring both
      are up to date. */
    function put_(x, o) {
      var self = this;
      // ensure the returned object from src is cached.
      return self.src.put(o).then(function(srcObj) {
        return self.delegate.put_(x, srcObj);
      });
    },

    /** Removes are sent to the cache and to the source, ensuring both
      are up to date. */
    function remove_(x, o) {
      var self = this;
      return self.src.remove(o).then(function() {
        return self.delegate.remove_(x, o);
      });
    },

    /** removeAll is executed on the cache and the source, ensuring both
      are up to date. */
    function removeAll_(x, skip, limit, order, predicate) {
      var self = this;
      return self.src.removeAll_(x, skip, limit, order, predicate).then(function() {
        return self.delegate.removeAll_(x, skip, limit, order, predicate);
      });
    },

    function cmd_(x, obj) {
      if ( foam.dao.DAO.PURGE_CMD === obj ) {
        this.onSrcReset();
      } else if ( this.PurgeRecordCmd.isInstance(obj) ) {
        // REVIEW: this.cache is a dao not object, need to call dao.remove(obj)?
        delete this.cache[obj.id];
      }

      return this.src.cmd_(x, obj);
    }
  ],

  listeners: [
    /** Keeps the cache in sync with changes from the source.
      @private */
    function onSrcPut(obj) {
      this.delegate.put(obj);
    },

    /** Keeps the cache in sync with changes from the source.
      @private */
    function onSrcRemove(obj) {
      this.delegate.remove(obj);
    },

    /** Keeps the cache in sync with changes from the source.
      @private */
    function onSrcReset() {
      this.clearPrivate_('delegate');

      // Not necessary, but frees up memory
      this.cache.removeAll();
    },

    /** Polls updates from the source. */
    function poll() {
      // No need to update if the tab is hidden
      if ( this.document.hidden ) return;

      if ( ! this.loginSuccess ) return;

      var self = this;

      self.delegate
        .orderBy(this.DESC(self.pollingProperty))
        .limit(1)
        .select().then(function(data) {
          if ( data.array.length === 1 ) {
            self.src
              .where(self.GT(self.pollingProperty, self.pollingProperty.f(data.array[0])))
              .select(self.QuickSink.create({ putFn: self.onSrcPut }));
          } else {
            // if there was nothing in the cache to delegate dao to
            // begin with, should also poll from src to get newly added objs
            self.src.select(self.QuickSink.create({ putFn: self.onSrcPut }))
          }
        });
    }
  ]
});
