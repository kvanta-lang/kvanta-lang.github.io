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
  const ctx = canvas.getContext("2d");
  let canvas_state = check_good_text(text).field;
  for (var i = 0; i < canvas_state.length; i++) {
    for (var j = 0; j < canvas_state[i].length; j++) {
      let r  = canvas_state[i][j] >> 16 & 255;
      let g  = canvas_state[i][j] >> 8 & 255;
      let b  = canvas_state[i][j] & 255;
      ctx.fillStyle = "rgba("+r+","+g+","+b+", 1)";
      ctx.fillRect( i, j, 1, 1);
    }
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
  parent: document.getElementById("code"),
});


console.log("Editor is ready!")
document.getElementById("runButton").addEventListener("click", run_code);
