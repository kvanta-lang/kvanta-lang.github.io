// CodeMirror bits (via esm.sh, no local install needed)
import { EditorView, lineNumbers, highlightActiveLine } from "https://esm.sh/@codemirror/view@6";
import { EditorState } from "https://esm.sh/@codemirror/state@6";
import { defaultKeymap, history, historyKeymap } from "https://esm.sh/@codemirror/commands@6";
import { indentOnInput } from "https://esm.sh/@codemirror/language@6";
import { oneDark } from "https://esm.sh/@codemirror/theme-one-dark@6";

// Language support (your Lezer parser compiled to quanta.js)
import { quanta } from "./quanta-support.js";

// Canvas runtime (drawScript + utilities)
import { drawScript, clearCanvas, checkIsCancelled, cancelNow } from "./canvas-runtime.js";

// WASM glue (wasm-pack output); adjust crate name/path
import initWasm, { Compiler } from "../quanta-lang/pkg/quanta_lang.js"; 

const runBtn = document.getElementById("runBtn");

let runtime = undefined;
let isRunning = false;

// starter code
const startCode = `animate();
setLineColor(Color::White);
for i in (0..200) {
   clear();
   line(0, 250, 1000, 250);
   circle(i, 200, 50);
   frame();
   sleep(10);
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

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function doStop() {
  (async() => {
    console.log("STOP!");
    runtime = undefined;
    cancelNow()
  })();
}

function doRun() {
  (async () => {
    try {
      cancelNow(false);
      isRunning = true;
      runBtn.disabled = true;
      clearCanvas();
      await initWasm();
      const src = editor.state.doc.toString();
      let compiler = Compiler.new();
      const compilation_result = compiler.compile_code(src);   // Rust returns drawing commands (string)
      if (compilation_result.error_code != 0) {
        alert(compilation_result.get_error_message());
        runBtn.disabled = false;
        return;
      }
      console.log("Compiling done");
      setRunningUI();
      runtime = compilation_result.get_runtime();
      runtime.execute();
      let need_continue = true;
      while(need_continue) {
        if (checkIsCancelled()) { return; }
        let blocks = runtime.get_commands();
        for (let i = 0; i < blocks.length; i++) {
          if (checkIsCancelled()) { return; }
          const block = blocks[i];
          let commands = block.get_commands();
           drawScript(commands, block.should_draw_frame);
           if (block.sleep_for >= 0) {
            await sleep(block.sleep_for);
           } else {
            need_continue = false;
            break;
           }
        }
      }             // render to Canvas2D
      //log("OK12\n" + script);
    } catch (e) {
      console.error(e);
      alert("Error: " + (e?.message ?? String(e)));
    } finally {
      setIdleUI();
      runBtn.disabled = false;
    }
  })();
}

function setRunningUI() {
  isRunning = true;
  runBtn.textContent = 'Stop';
  runBtn.dataset.state = 'stop';
  runBtn.disabled = false;
}

function setIdleUI() {
  isRunning = false;
  runBtn.textContent = 'Run (Ctrl/Cmd+Enter)';
  runBtn.dataset.state = 'run';
  runBtn.disabled = false;
}

runBtn.addEventListener('click', () => {
  if (!isRunning) {
    doRun();
  } else {
    doStop();
  }
});

// Ctrl/Cmd+Enter
addEventListener("keydown", (e) => {
  const isMac = navigator.platform.toLowerCase().includes("mac");
  if ((isMac ? e.metaKey : e.ctrlKey) && e.key === "Enter") {
    e.preventDefault();
    doRun();
  }
});

editor.focus();
