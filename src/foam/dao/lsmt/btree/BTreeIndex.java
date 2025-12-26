/**
* @license
* Copyright 2025 Google Inc. All Rights Reserved.
* http://www.apache.org/licenses/LICENSE-2.0
*/
package foam.dao.lsmt.btree;

import foam.mlang.order.Comparator;
import foam.mlang.predicate.*;
import foam.dao.Sink;
import foam.dao.index.*;
import foam.mlang.sink.Count;
import foam.mlang.sink.GroupBy;
import foam.lang.FObject;
import foam.lang.PropertyInfo;

public class BTreeIndex extends foam.dao.index.AbstractIndex {
  protected PropertyInfo  propertyInfo_;
  // protected 

  public BTreeIndex(PropertyInfo info) {
    propertyInfo_ = info;
  }
  
  public Object put(Object btree, FObject value) {
    if ( btree == null ) btree = BTree.empty();
    return BTreeUpdate.merge(this.propertyInfo_, BTreeUpdate.NO_OP, (Object[]) btree, BTree.singleton(value));
  }

  public FObject find(Object state, Object key) {
    return (FObject) BTree.find(this.propertyInfo_, (Object[]) state, key);
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

  @Override
  public SelectPlan planSelect(Object state, Sink sink, long skip, long limit, Comparator order, Predicate predicate) {
    throw new RuntimeException("AA");
  }
}