// quanta-highlighting.ts (your language package)
import { styleTags, tags as t } from "@lezer/highlight";

export const quantaStyleTags = styleTags({
  // control flow:
  "if else while for return": t.controlKeyword,

  // decl/definition-ish:
  "func global const": t.definitionKeyword,

  // types:
  "int float color bool array": t.typeName,

  // identifiers & literals:
  Identifier: t.variableName,
  Number: t.number,
  String: t.string,

  // comments:
  LineComment: t.lineComment,
  BlockComment: t.blockComment,

  // operators/punctuation/brackets:
  "=": t.operator,
  "== != < <= > >=": t.compareOperator,
  "&& || !": t.logicOperator,
  "+" : t.arithmeticOperator,
  "-": t.arithmeticOperator,
  "* / %": t.arithmeticOperator,
  ", ;": t.separator,
  "( )": t.paren,
  "{ }": t.brace,
  "[ ]": t.squareBracket,
});