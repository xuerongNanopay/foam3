/**
 * @license
 * Copyright 2025 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.ANTLR4({
  package: 'foam.dao.fql.antlr',
  name: 'Fql',
  antlr: `
grammar Fql;

@header {
package foam.dao.fql.antlr;
}

//----------------------
// Parser rules
//----------------------
expr: expr ('*'|'/') expr      # MulDiv
    | expr ('+'|'-') expr      # AddSub
    | INT                      # Int
    | '(' expr ')'             # Parens
    ;

//----------------------
// Lexer rules
//----------------------
INT : [0-9]+;                  // integer number
WS  : [ \\t\\r\\n]+ -> skip;      // ignore spaces and newlines
  `
})