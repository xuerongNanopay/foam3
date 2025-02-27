/**
 * @license
 * Copyright 2018 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.nanos.logger',
  name: 'AbstractLogger',
  implements: [ 'foam.nanos.logger.Logger' ],

  abstract: true,

  javaImports: [
    'java.io.PrintWriter',
    'java.io.StringWriter',
    'java.io.Writer'
  ],

  javaCode: `
    protected ThreadLocal<StringBuilder> sb = new ThreadLocal<StringBuilder>() {
      @Override
      protected StringBuilder initialValue() {
        return new StringBuilder();
      }

      @Override
      public StringBuilder get() {
        StringBuilder b = super.get();
        b.setLength(0);
        return b;
      }
    };
  `,

  methods: [
    {
      name: 'formatArg',
      type: 'String',
      args: [
        {
          name: 'obj',
          type: 'Any'
        }
      ],
      javaCode:
`if ( obj instanceof Throwable ) {
  Throwable   t  = (Throwable) obj;
  Writer      w  = new StringWriter();
  PrintWriter pw = new PrintWriter(w);

  t.printStackTrace(pw);

  return w.toString();
}
return String.valueOf(obj);`
    },
    {
      name: 'combine',
      type: 'String',
      args: [
        {
          name: 'args',
          type: 'Any[]'
        }
      ],
      javaCode:
      `
  StringBuilder str = sb.get();
  if ( args.length == 1 && args[0] instanceof Throwable ) {
    str.append(((Throwable) args[0]).getMessage());
    str.append(formatArg(args[0]));
    return str.toString();
  }

  if ( args.length > 0) {
    str.append(formatArg(args[0]));
  }
  for ( int i = 1; i < args.length; ++i) {
    Object n = args[i];
    str.append(',');
    str.append(formatArg(n));
  }
  return str.toString();`
    }
  ]
});
