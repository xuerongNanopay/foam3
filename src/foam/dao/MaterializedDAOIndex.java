/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

package foam.dao;

import foam.core.FObject;
import foam.dao.index.*;
import foam.dao.Sink;
import foam.mlang.order.Comparator;
import foam.mlang.predicate.Predicate;


public class MaterializedDAOIndex
  extends AbstractIndex
{
  protected final MaterializedDAO dao_;

  public MaterializedDAOIndex(MaterializedDAO dao) {
    dao_ = dao;
  }

  public Object put(Object state, FObject value) {
    foam.nanos.logger.StdoutLogger.instance().info("MaterializedDAOIndex.put",value);
    try {
      return dao_.indexPut(state, value);
    } catch (Throwable t) {
      foam.nanos.logger.StdoutLogger.instance().info("MaterializedDAOIndex.put",value, t);
      throw new RuntimeException(t);
    }
  }

  // Remove an object
  public Object remove(Object state, FObject value) {
    return dao_.indexRemove(state, value);
  }

  // Remove all objects
  public Object removeAll() {
    return dao_.indexRemoveAll();
  }

  public FObject find(Object state, Object key) {
    return null;
  }

  // Create a Plan for a select()
  public SelectPlan planSelect(Object state, Sink sink, long skip, long limit, Comparator order, Predicate predicate) {
    return NoPlan.instance();
  }

  // Return number of objects stored in this Index
  public long size(Object state) {
    return 0;
  }
}
