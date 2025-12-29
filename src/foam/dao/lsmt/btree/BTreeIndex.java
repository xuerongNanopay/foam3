/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import java.util.Comparator;
import foam.mlang.predicate.*;
import foam.dao.Sink;
import foam.dao.index.*;
import foam.mlang.sink.Count;
import foam.mlang.sink.GroupBy;
import foam.lang.FObject;
import foam.lang.PropertyInfo;

public class BTreeIndex extends foam.dao.index.AbstractIndex {
  final protected PropertyInfo  propertyInfo_;
  // protected 
  final protected Comparator<Object> asymmetricComparator_;

  public BTreeIndex(PropertyInfo info) {
    propertyInfo_ = info;
    asymmetricComparator_ = (f, k) -> (-propertyInfo_.comparePropertyToObject(k, f));
  }
  
  public Object put(Object btree, FObject value) {
    if ( btree == null ) btree = BTree.empty();
    return BTreeUpdate.merge(this.propertyInfo_, BTreeUpdate.NO_OP, (Object[]) btree, BTree.singleton(value));
  }

  public FObject find(Object state, Object key) {
    return (FObject) BTree.find(this.asymmetricComparator_, (Object[]) state, key);
  }

  public Object remove(Object state, FObject value) {
    throw new RuntimeException("AA");
  }

  public Object removeAll() {
    throw new RuntimeException("AA");
  }

  public long size(Object btree) {
    return BTree.size((Object[]) btree);
  }

  public SelectPlan planSelect(Object state, Sink sink, long skip, long limit, foam.mlang.order.Comparator order, Predicate predicate) {
    throw new RuntimeException("AA");
  }
}