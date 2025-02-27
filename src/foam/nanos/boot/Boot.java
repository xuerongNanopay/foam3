/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

package foam.nanos.boot;

import foam.core.*;
import foam.dao.AbstractSink;
import foam.dao.ArraySink;
import foam.dao.DAO;
import foam.dao.java.JDAO;
import foam.dao.ProxyDAO;
import foam.nanos.auth.Group;
import foam.nanos.auth.Subject;
import foam.nanos.auth.User;
import foam.nanos.logger.Logger;
import foam.nanos.logger.ProxyLogger;
import foam.nanos.logger.StdoutLogger;
import foam.nanos.script.Script;
import foam.nanos.session.Session;
import foam.util.SafetyUtil;
import java.lang.Exception;
import java.util.*;
import static foam.mlang.MLang.EQ;

public class Boot {
  // Context key used to store the top-level root context in the context.
  public final static String ROOT      = "_ROOT_";
  public final static String BOOT_TIME = "BOOT_TIME";

  protected DAO                       serviceDAO_;
  protected X                         root_      = new MutableX();
  protected Map<String, NSpecFactory> factories_ = new HashMap<>();

  public Boot() {
    this("");
  }

  public Boot(String datadir) {
    XLocator.set(root_);

    Logger logger = new ProxyLogger(StdoutLogger.instance());
    root_.put("logger", logger);
    root_.put(BOOT_TIME, System.currentTimeMillis());

    boolean cluster = SafetyUtil.equals("true", System.getProperty("CLUSTER", "false"));

    if ( SafetyUtil.isEmpty(datadir) ) {
      datadir = System.getProperty("JOURNAL_HOME");
    }

    foam.nanos.fs.Storage storage = new foam.nanos.fs.FileSystemStorage(datadir);
    root_.put(foam.nanos.fs.FileSystemStorage.class, storage);

    var readStorage = storage;
    if ( ! SafetyUtil.isEmpty(System.getProperty("resource.journals.dir")) ) {
      readStorage = new foam.nanos.fs.ResourceStorage(System.getProperty("resource.journals.dir"));
    }
    root_.put(foam.nanos.fs.Storage.class, readStorage);


    // Used for all the services that will be required when Booting
    foam.dao.MDAO mdao = new foam.dao.MDAO(NSpec.getOwnClassInfo());
    mdao.addIndex(NSpec.SERVE);
    serviceDAO_ = new JDAO(((foam.core.ProxyX) root_).getX(), mdao, "services", cluster);
    serviceDAO_ = new foam.nanos.auth.PermissionedPropertyDAO(root_, serviceDAO_);

    installSystemUser();

    // Just adding services in order will create an un-ordered tree,
    // so add so that we get a balanced Context tree.
    ArraySink arr = (ArraySink) serviceDAO_.select(new ArraySink());
    List      l   = perfectList(arr.getArray());

    // Record all sub contexts to be frozen along with the root context
    var subContexts = new HashSet<String>();
    for ( int i = 0 ; i < l.size() ; i++ ) {
      NSpec sp = (NSpec) l.get(i);
      if ( ! sp.getEnabled() ) {
        logger.info("Disabled", sp.getName());
        continue;
      }

      var x      = root_;
      var path   = sp.getName().split("/");
      var parent = new StringBuilder();

      // Register path as sub context
      for ( int j = 0 ; j < path.length - 1 ; j++ ) {
        var contextName = path[j];
        if ( x.get(contextName) == null ) {
          var subX = new SubX(Boot.this::getX, parent.toString());
          x.put(contextName, subX);
        }
        x = (X) x.get(contextName);

        if ( parent.length() > 0 ) parent.append("/");
        parent.append(contextName);
        subContexts.add(parent.toString());
      }

      // Register service
      var serviceName = path[path.length - 1];
      NSpecFactory factory = new NSpecFactory((ProxyX) x, sp);
      factories_.put(sp.getName(), factory);
      logger.info("Registering", sp.getName());
      x.putFactory(serviceName, factory);

      // Register link to the service added to the sub context, allowing the
      // service to be accessible via both the sub context and the root context.
      // Eg. "foo/test" nspec can be instantiated by
      //
      //      x.cd("foo").get("test");
      // or
      //      x.get("foo/test");
      //
      if ( x != root_ )
        root_.putFactory(sp.getName(), factory);
    }

    serviceDAO_.listen(new AbstractSink() {
      @Override
      public void put(Object obj, Detachable sub) {
        NSpec sp = (NSpec) obj;
        factories_.get(sp.getName()).invalidate(sp);
      }
    }, null);

    // Use an XFactory so that the root context can contain itself.
    root_.putFactory(ROOT, new XFactory() {
      public Object create(X x) {
        return Boot.this.getX();
      }
    });

    root_.putFactory("user", new XFactory() {
      public Object create(X x) {
        logger.warning(new Exception("Deprecated use of x.get(\"user\")"));
        return ((Subject) x.get("subject")).getUser();
      }
    });

    root_.putFactory("agent", new XFactory() {
      public Object create(X x) {
        logger.warning(new Exception("Deprecated use of x.get(\"agent\")"));
        return ((Subject) x.get("subject")).getRealUser();
      }
    });

    // Freeze sub contexts
    for ( var path : subContexts ) {
      var x = root_.cd(path);
      if ( x instanceof SubX ) {
        ((SubX) x).freeze();
      }
    }

    // Revert root_ to non ProxyX to avoid letting children add new bindings.
    root_ = ((ProxyX) root_).getX();
    XLocator.set(root_);

    if ( cluster ) {
      // On startup, select() above will be against repo services.0.
      // Mediator/Node replay put()s will hit the serviceDAO_ above,
      // which has a listener to Reload the service on change.
      foam.nanos.medusa.MedusaSupport support = (foam.nanos.medusa.MedusaSupport) root_.get("medusaSupport");
      if ( support != null ) {
        serviceDAO_ = support.clusterServiceDAO(root_, serviceDAO_);
      }
    }

    // Export the ServiceDAO
    ((ProxyDAO) root_.get("nSpecDAO")).setDelegate(
      new foam.nanos.auth.AuthorizationDAO.Builder(getX())
        .setDelegate(serviceDAO_)
        .setAuthorizer(new foam.nanos.auth.AuthorizableAuthorizer("service"))
        .build());

    String startScript = System.getProperty("foam.main", "main");
    if ( startScript != null ) {
      DAO scriptDAO = (DAO) root_.get("bootScriptDAO");
      if ( scriptDAO == null ) {
        logger.warning("DAO Not Found: bootScriptDAO. Falling back to scriptDAO");
        scriptDAO = (DAO) root_.get("scriptDAO");
      }
      Script script = (Script) scriptDAO.find(startScript);
      if ( script != null ) {
        logger.info("Boot,script", startScript);
        ((Script) script.fclone()).runScript(root_);
      } else {
        logger.warning("Boot, Script not found", startScript);
      }
    }

    serviceDAO_.where(EQ(NSpec.LAZY, false)).select(new AbstractSink() {
      @Override
      public void put(Object obj, Detachable sub) {
        NSpec sp = (NSpec) obj;

        logger.info("Invoking Service", sp.getName());
        root_.get(sp.getName());
      }
    });
  }

  protected List perfectList(List src) {
    List dst = new ArrayList(src.size());
    perfectList(src, dst, 0, src.size()-1);
    return dst;
  }

  protected void perfectList(List src, List dst, int start, int end) {
    if ( start == end ) {
      dst.add(src.get(start));
    } else if ( end > start ) {
      int pivot = ( start + end ) / 2;
      perfectList(src, dst, pivot, pivot);
      perfectList(src, dst, start, pivot-1);
      perfectList(src, dst, pivot+1, end);
    }
  }

  protected void installSystemUser() {
    User user = new User();
    user.setId(User.SYSTEM_USER_ID);
    user.setSpid("foam");
    user.setFirstName("system");
    user.setGroup("system");
    user.setLoginEnabled(false);

    Session session = new Session();
    session.setUserId(user.getId());
    session.setContext(root_);

    Subject subject = new Subject();
    subject.setUser(user);
    root_.put("subject", subject);
    root_.put(Session.class, session);

    Group group = new Group();
    group.setId("system");
    root_.put("group", group);
  }

  public X getX() { return root_; }

  public static void main (String[] args)
    throws java.lang.Exception
  {
    System.out.println("Starting Nanos Server");

    boolean datadirFlag = false;

    String datadir = "";
    for ( int i = 0 ; i < args.length ; i++ ) {
      String arg = args[i];

      if ( datadirFlag ) {
        datadir = arg;
        datadirFlag = false;
      } else if ( arg.equals("--datadir") ) {
        datadirFlag = true;
      } else {
        System.err.println("Unknown argument " + arg);
        System.exit(1);
      }
    }

    System.out.println("Datadir is " + datadir);

    new Boot(datadir);
  }
}
