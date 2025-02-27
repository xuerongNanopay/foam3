/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

package foam.core;

import foam.lib.parse.ParserContextImpl;
import foam.lib.parse.StringPStream;

import javax.xml.stream.XMLStreamReader;
import java.nio.ByteBuffer;
import java.security.MessageDigest;
import java.security.Signature;
import java.security.SignatureException;
import java.util.Date;

public abstract class AbstractDatePropertyInfo
    extends AbstractPropertyInfo
{
  protected static final ThreadLocal<ByteBuffer> bb = new ThreadLocal<ByteBuffer>() {
    @Override
    protected ByteBuffer initialValue() {
      return ByteBuffer.wrap(new byte[8]);
    }

    @Override
    public ByteBuffer get() {
      ByteBuffer bb = super.get();
      bb.clear();
      return bb;
    }
  };

  public int compareValues(java.lang.Object o1, java.lang.Object o2) {
    return ((Date)o1).compareTo(((Date)o2));
  }

  public Object fromString(String value) {
    StringPStream ps = new StringPStream(value);
    ParserContextImpl x = new ParserContextImpl();
    ps = (StringPStream) jsonParser().parse(ps, x);
    return ps == null ? null : ps.value();
  }

  @Override
  public void cloneProperty(FObject source, FObject dest) {
    set(dest, get(source));
  }

  @Override
  public void updateDigest(FObject obj, MessageDigest md) {
    if ( ! includeInDigest() ) return;
    Date date = (Date) get(obj);
    if ( date == null ) return;

    long val = date.getTime();
    md.update((ByteBuffer) bb.get().putLong(val).flip());
  }

  @Override
  public void toJSON(foam.lib.json.Outputter outputter, Object value) {
    if ( value == null ) {
      outputter.output(null);
    } else {
      outputter.outputDateValue((Date)value);
    }
  }

  @Override
  public void updateSignature(FObject obj, Signature sig) throws SignatureException {
    if ( ! includeInSignature() ) return;
    Date date = (Date) get(obj);
    if ( date == null ) return;

    long val = date.getTime();
    sig.update((ByteBuffer) bb.get().putLong(val).flip());
  }

  protected abstract java.util.Date get_(Object o);
  protected abstract java.util.Date cast(Object key);

//  public foam.lib.parse.Parser jsonParser() {
//    return foam.lib.json.DateParser.instance() == null ? foam.lib.json.DateParser.instance(): null;
//  }

  public int compare(Object o1, Object o2) {
    return foam.util.SafetyUtil.compare(get_(o1), get_(o2));
  }

  public int comparePropertyToObject(Object key, Object o) {
    return foam.util.SafetyUtil.compare(cast(key), get_(o));
  }

  public int comparePropertyToValue(Object key, Object value) {
    return foam.util.SafetyUtil.compare(cast(key), cast(value));
  }

  public foam.lib.parse.Parser queryParser() {
    return foam.lib.query.DuringExpressionParser.instance();
  }

  public foam.lib.parse.Parser csvParser() {
    return foam.lib.json.DateParser.instance();
  }

  public Class getValueClass() {
    return java.util.Date.class;
  }

  public boolean isDefaultValue(Object o) {
    return foam.util.SafetyUtil.compare(get_(o), null) == 0;
  }

  public void format(foam.lib.formatter.FObjectFormatter formatter, foam.core.FObject obj) {
    formatter.output(get_(obj));
  }
}
