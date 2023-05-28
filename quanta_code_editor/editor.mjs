import {EditorView, basicSetup} from "codemirror"
import {quanta} from "./dist/index.js"
import {keymap} from "@codemirror/view"
import {acceptCompletion} from "@codemirror/autocomplete"
import {check_good_text} from "../www/quanta.js"

let sync_val = "";



function run_code() {
  console.log("Running code...")
  let text = editor.state.doc.toString().trim();
  console.log("text: " + text);
  let canvas = document.getElementById("drawing");
  if (check_good_text(text)) {
      canvas.style.backgroundColor = "#00FF00"
      canvas.innerText = "It is a valid program!";
  } else {
      canvas.style.backgroundColor = "#FF0000"
      canvas.innerText = "It is a not valid program :(";
  }
}

console.log("Hello from Quanta Code Editor!")
let editor = new EditorView({
  extensions: [
    basicSetup,
    quanta(),
    keymap.of([{key: "Tab", run: acceptCompletion}]),
    EditorView.updateListener.of(
      function(e) {
        sync_val = e.state.doc.toString();
      }
    ),
  ],
  doc: "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n",
  parent: document.getElementById("code")
});


console.log("Editor is ready!")
document.getElementById("runButton").addEventListener("click", run_code);
