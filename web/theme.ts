// quanta-theme.ts
import { EditorView } from "@codemirror/view";
import { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

/** Editor chrome (selection, caret, gutters) */
export const quantaEditorTheme = EditorView.theme(
  {
    "&": { color: "var(--qt-fg)", backgroundColor: "var(--qt-bg)" },
    ".cm-content": { caretColor: "var(--qt-caret)" },
    "&.cm-editor.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": {
      backgroundColor: "var(--qt-selection)"
    },
    ".cm-gutters": {
      backgroundColor: "var(--qt-gutter-bg)",
      color: "var(--qt-gutter-fg)",
      border: "none"
    }
  },
  { dark: true } // set `true` if your default palette is dark
);

/** Syntax colors per tag */
export const quantaHighlightStyle = HighlightStyle.define([
  // keywords
  { tag: t.controlKeyword, color: "var(--qt-keyword-ctrl)", fontWeight: "600" },
  { tag: t.keyword, color: "var(--qt-keyword)" },
  { tag: t.definitionKeyword, color: "var(--qt-keyword-def)", fontWeight: "600" },

  // types & defs
  { tag: t.typeName, color: "var(--qt-type)", fontStyle: "italic" },
  { tag: t.definition(t.variableName), color: "var(--qt-def)", fontWeight: "600" },
  { tag: t.variableName, color: "var(--qt-var)" },
  { tag: t.function(t.variableName), color: "var(--qt-fn)" },
  { tag: t.propertyName, color: "var(--qt-prop)" },

  // literals
  { tag: [t.string, t.special(t.string)], color: "var(--qt-str)" },
  { tag: t.number, color: "var(--qt-num)" },
  { tag: t.bool, color: "var(--qt-bool)", fontWeight: "600" },

  // operators / punctuation / brackets
  { tag: [t.operator, t.compareOperator, t.logicOperator, t.arithmeticOperator], color: "var(--qt-op)" },
  { tag: [t.punctuation, t.separator], color: "var(--qt-punc)" },
  { tag: [t.paren, t.brace, t.squareBracket], color: "var(--qt-bracket)" },

  // comments & misc
  { tag: t.comment, color: "var(--qt-comment)", fontStyle: "italic" },
  { tag: t.invalid, color: "var(--qt-error)", textDecoration: "wavy underline" },
  { tag: t.meta, color: "var(--qt-meta)" }
]);

export const quantaSyntax = syntaxHighlighting(quantaHighlightStyle);