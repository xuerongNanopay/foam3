/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
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

foam.CLASS({
  package: 'foam.box',
  name: 'HTTPReplyBox',
  implements: ['foam.box.Box'],

  javaImports: [
    'java.nio.charset.StandardCharsets'
  ],


  imports: [
    // Optional import.
    //    'httpResponse'
  ],

  javaCode: `
    protected static final ThreadLocal<foam.lib.formatter.FObjectFormatter> formatter_ = new ThreadLocal<foam.lib.formatter.FObjectFormatter>() {
      @Override
      protected foam.lib.formatter.JSONFObjectFormatter initialValue() {
        foam.lib.formatter.JSONFObjectFormatter formatter = new foam.lib.formatter.JSONFObjectFormatter();
        // Needed because JS JSON parser doesn't support unquoted keys.
        formatter.setQuoteKeys(true);
        formatter.setOutputShortNames(true);
        formatter.setOutputDefaultValues(false);
        formatter.setPropertyPredicate(new foam.lib.AndPropertyPredicate(new foam.lib.PropertyPredicate[] {new foam.lib.NetworkPropertyPredicate(), new foam.lib.PermissionedPropertyPredicate()}));
        return formatter;
      }

      @Override
      public foam.lib.formatter.FObjectFormatter get() {
        foam.lib.formatter.FObjectFormatter formatter = super.get();
        formatter.reset();
        return formatter;
      }
    };
  `,

  methods: [
    {
      name: 'send',
      code: function(m) {
        throw 'unimplemented';
      },
      swiftCode: 'throw FoamError("unimplemented")',
      javaCode: `
try {
  jakarta.servlet.http.HttpServletResponse resp = (jakarta.servlet.http.HttpServletResponse) getX().get("httpResponse");
  resp.setContentType("application/json");

  java.io.PrintWriter                 writer    = resp.getWriter();
  foam.lib.formatter.FObjectFormatter formatter = formatter_.get();

  formatter.setX(getX());
  formatter.output(msg);
  formatter.setX(null); // avoid retaining reference to X

  StringBuilder builder = formatter.builder();
  resp.setContentLengthLong(builder.toString().getBytes(StandardCharsets.UTF_8).length);

  writer.append(builder);
  writer.flush();
} catch(java.io.IOException e) {
  throw new RuntimeException(e);
}
`
    }
  ]
});
