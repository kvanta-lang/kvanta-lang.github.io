import {EditorView, basicSetup} from "codemirror"
import {quanta} from "./dist/index.js"
import {keymap} from "@codemirror/view"
import {acceptCompletion} from "@codemirror/autocomplete"

let sync_val = "";

function is_good_text(text) {
  return text === "function(a, b)"
}

function run_code() {
  console.log("Running code...")
  let text = editor.state.doc.toString().trim();
  console.log("text: " + text);
  let canvas = document.getElementById("drawing");
  if (is_good_text(text) != 0) {
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
        console.log("sync_val: " + sync_val);
        // run_code();
      }
    ),
  ],
  doc: "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n",
  parent: document.getElementById("code")
});


console.log("Editor is ready!")
document.getElementById("runButton").addEventListener("click", run_code);
