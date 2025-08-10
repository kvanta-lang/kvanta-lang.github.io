// CodeMirror bits (via esm.sh, no local install needed)
import { EditorView, lineNumbers, highlightActiveLine } from "https://esm.sh/@codemirror/view@6";
import { EditorState } from "https://esm.sh/@codemirror/state@6";
import { defaultKeymap, history, historyKeymap } from "https://esm.sh/@codemirror/commands@6";
import { indentOnInput } from "https://esm.sh/@codemirror/language@6";
import { oneDark } from "https://esm.sh/@codemirror/theme-one-dark@6";

// Language support (your Lezer parser compiled to quanta.js)
import { quanta } from "./quanta-support.js";

// Canvas runtime (drawScript + utilities)
import { drawScript, log } from "./canvas-runtime.js";

// WASM glue (wasm-pack output); adjust crate name/path
import initWasm, { compile_code } from "../quanta-lang/pkg/quanta_lang.js"; 

const runBtn = document.getElementById("runBtn");

// starter code
const startCode = `circle(320, 240, 100);
setFigureColor(Color::Red);
setLineColor(Color::Green);
circle(320, 240, 50);


rectangle(100, 100, 200, 200);

setFigureColor(Color::Blue);
rectangle(125, 125, 175, 175);

for i in (0..10) {
    setFigureColor(Color::Random);
    int a = 50;
    rectangle(0, i * a, (i+1) * a, (i+1) * a);
}
`;

const editor = new EditorView({
  state: EditorState.create({
    doc: startCode,
    extensions: [
      lineNumbers(),
      highlightActiveLine(),
      indentOnInput(),
      history(),
      EditorView.updateListener.of(v => { /* hooks later */ }),
      oneDark
    ]
  }),
  parent: document.getElementById("editor")
});

function doRun() {
  (async () => {
    try {
      runBtn.disabled = true;
      //log("Compiling…");
      console.log("Compiling");
      await initWasm();
      console.log("Init wasm done");
      const src = editor.state.doc.toString();
      const script = compile_code(src);     // Rust returns drawing commands (string)
      console.log("Compiling done");
      drawScript(script);              // render to Canvas2D
      //log("OK12\n" + script);
    } catch (e) {
      console.error(e);
      log("Error: " + (e?.message ?? String(e)));
    } finally {
      runBtn.disabled = false;
    }
  })();
}
runBtn.addEventListener("click", doRun);

// Ctrl/Cmd+Enter
addEventListener("keydown", (e) => {
  const isMac = navigator.platform.toLowerCase().includes("mac");
  if ((isMac ? e.metaKey : e.ctrlKey) && e.key === "Enter") {
    e.preventDefault();
    doRun();
  }
});

editor.focus();
