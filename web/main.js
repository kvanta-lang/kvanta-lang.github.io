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
const startCode = `// Quanta demo
background #071022
circle 200 160 80 fill=tomato stroke=white width=4
rectangle 320 80 200 120 fill=#1e293b stroke=#94a3b8 width=2
line 60 300 540 300 stroke=#22d3ee width=3
polygon 380 260 430 340 330 340 fill=#10b981
arc 200 160 110 20 320 stroke=gold width=6
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
