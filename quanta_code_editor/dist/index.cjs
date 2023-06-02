'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var parser = require('../quanta/src/parser');
var language = require('@codemirror/language');
var highlight = require('@lezer/highlight');
var autocomplete = require('@codemirror/autocomplete');

var QuantaLanguage = language.LRLanguage.define({
    parser: parser.parser.configure({
        props: [
            language.indentNodeProp.add({
                Application: language.delimitedIndent({ closing: ")", align: false })
            }),
            language.foldNodeProp.add({
                Application: language.foldInside
            }),
            highlight.styleTags({
                Identifier: highlight.tags.variableName,
                Boolean: highlight.tags.bool,
                String: highlight.tags.string,
                LineComment: highlight.tags.lineComment,
                "( )": highlight.tags.paren
            })
        ]
    }),
    languageData: {
        commentTokens: { line: ";" }
    }
});
var quantaCompletion = QuantaLanguage.data.of({
    autocomplete: autocomplete.completeFromList([
        { label: "bool", type: "keyword" },
        { label: "int", type: "keyword" },
        { label: "float", type: "keyword" },
        { label: "Color", type: "keyword" },
        { label: "Green", type: "keyword" },
        { label: "Blue", type: "keyword" },
        { label: "Red", type: "keyword" },
        { label: "Cyan", type: "keyword" },
        { label: "Pink", type: "keyword" },
        { label: "Black", type: "keyword" },
        { label: "White", type: "keyword" },
        { label: "circle", type: "function" },
        { label: "rectangle", type: "function" },
        { label: "line", type: "function" },
        { label: "setLineColor", type: "function" },
        { label: "setFigureColor", type: "function" },
    ])
});
function quanta() {
    return new language.LanguageSupport(QuantaLanguage, [quantaCompletion]);
}

exports.QuantaLanguage = QuantaLanguage;
exports.quanta = quanta;
exports.quantaCompletion = quantaCompletion;
