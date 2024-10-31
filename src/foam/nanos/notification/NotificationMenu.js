foam.CLASS({
   package: 'foam.nanos.notification',
   name: 'NotificationMenu',
   extends: 'foam.nanos.menu.Menu',


   properties: [
   {
      name: 'readPredicate',
      initObject: function(o) {
      /* ignoreWarning */
      o.readPredicate = foam.mlang.predicate.Func.create({
         fn: async function(o) {
               if ((await o.__subContext__.pushRegistryAgent.currentState.promise) == 'DEFAULT') {
                  return true;
               } 
               return false;
               }
            }, this)
         }
      }
   ],
})