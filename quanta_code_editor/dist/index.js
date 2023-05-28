import { parser } from '../quanta/src/parser';
import { LRLanguage, indentNodeProp, delimitedIndent, foldNodeProp, foldInside, LanguageSupport } from '@codemirror/language';
import { styleTags, tags } from '@lezer/highlight';
import { completeFromList } from '@codemirror/autocomplete';

var QuantaLanguage = LRLanguage.define({
    parser: parser.configure({
        props: [
            indentNodeProp.add({
                Application: delimitedIndent({ closing: ")", align: false })
            }),
            foldNodeProp.add({
                Application: foldInside
            }),
            styleTags({
                Identifier: tags.variableName,
                Boolean: tags.bool,
                String: tags.string,
                LineComment: tags.lineComment,
                "( )": tags.paren
            })
        ]
    }),
    languageData: {
        commentTokens: { line: ";" }
    }
});
var quantaCompletion = QuantaLanguage.data.of({
    autocomplete: completeFromList([
        { label: "function", type: "keyword" },
        { label: "bool", type: "keyword" },
        { label: "int", type: "keyword" },
        { label: "circle", type: "function" },
        { label: "rectangle", type: "function" },
        { label: "line", type: "function" }
    ])
});
function quanta() {
    return new LanguageSupport(QuantaLanguage, [quantaCompletion]);
}

export { QuantaLanguage, quanta, quantaCompletion };
