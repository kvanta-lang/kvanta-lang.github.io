import {EditorView, basicSetup} from "codemirror"
import {quanta} from "./dist/index.js"
import {keymap} from "@codemirror/view"
import {acceptCompletion} from "@codemirror/autocomplete"


let editor = new EditorView({
  extensions: [
    basicSetup, 
    quanta(),
    keymap.of([{key: "Tab", run: acceptCompletion}]),
  ],
  parent: document.body
})
