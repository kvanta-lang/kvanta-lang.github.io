// CodeMirror bits (via esm.sh, no local install needed)
//import { EditorView, lineNumbers, highlightActiveLine } from "@codemirror/view";
//import { EditorState } from "@codemirror/state";
//import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
//import { indentOnInput } from "@codemirror/language";
import { oneDark, oneDarkHighlightStyle } from "@codemirror/theme-one-dark";
import {barf, dracula} from 'thememirror';
//import { autocompletion } from "@codemirror/autocomplete";
import {EditorState, RangeSetBuilder, EditorSelection} from "@codemirror/state"
import { HighlightStyle, tags as t } from "@codemirror/highlight";

import {
  EditorView, keymap, highlightSpecialChars, drawSelection,
  highlightActiveLine, dropCursor, rectangularSelection,
  crosshairCursor, lineNumbers, highlightActiveLineGutter,
  Decoration, ViewPlugin
} from "@codemirror/view"
import {
  defaultHighlightStyle, syntaxHighlighting, indentOnInput,
  bracketMatching, foldGutter, foldKeymap, indentUnit
} from "@codemirror/language"
import {
  defaultKeymap, history, historyKeymap
} from "@codemirror/commands"
import {
  autocompletion, closeBrackets,
  closeBracketsKeymap, completionKeymap
} from "@codemirror/autocomplete"
import { linter, setDiagnostics } from "@codemirror/lint";
// Language support (your Lezer parser compiled to quanta.js)
import { quanta, quantaSyntax, quantaLanguageSupport } from "./quanta-support.ts";

import { quantaTheme } from "./custom-theme";

// Canvas runtime (drawScript + utilities)
import { drawScript, setup, checkIsCancelled, cancelNow } from "./canvas-runtime.js";

// WASM glue (wasm-pack output); adjust crate name/path
import initWasm, { Compiler } from "../quanta-lang/pkg/quanta_lang.js"; 
//import { rustHighlighting } from "../grammar/highlight.js";

const runBtn = document.getElementById("runBtn");

let runtime = undefined;
let isRunning = false;

const fourSpaceIndent = indentUnit.of("    "); // 4 spaces

const insertFourSpaces = keymap.of([{
  key: "Tab",
  run: ({ state, dispatch }) => {
    dispatch(
      state.replaceSelection("    ") // 4 spaces
    );
    return true; // handled
  }
}]);

const newlineSameIndent = keymap.of([{
  key: "Enter",
  run: (view) => {
    const { state } = view;
    const tr = state.changeByRange(range => {
      const line = state.doc.lineAt(range.head);
      let leadingWS = (line.text.match(/^[ \t]*/) || [""])[0]; // copy tabs/spaces exactly
      let extra = "";
      if (line.text.trimEnd().endsWith("{")) {
        // increase indent after {
        leadingWS += "    "; // add 4 spaces
      }
      if (line.text.trimEnd().endsWith("{}")) {
        // increase indent after {
        extra += "\n" + leadingWS; // add 4 spaces
        leadingWS += "    "
      }
      const insert = "\n" + leadingWS + extra;
      return {
        changes: { from: range.from, to: range.to, insert },
        range: EditorSelection.cursor(range.from + leadingWS.length + 1)
      };
    });
    view.dispatch(tr, { userEvent: "input" });
    return true;
  }
}]);

function showError(editor, err) {
  let diagnostics = [];
  const from_line = editor.state.doc.line(err.start_row);
  const from = Math.min(from_line.to - 1, from_line.from + err.start_column - 1);
  const to_line = editor.state.doc.line(err.end_row);
  const to = Math.min(to_line.to, to_line.from + err.end_column); 
  diagnostics.push({
    from: from,
    to: to, // adjust for token length if needed
    severity: "error",
    message: err.get_error_message()
  });

  editor.dispatch(setDiagnostics(editor.state, diagnostics));
}

function showOk(editor) {
  editor.dispatch(setDiagnostics(editor.state, []));
}

const STORAGE_KEY = "quanta-editor-code";

const savedCode = localStorage.getItem(STORAGE_KEY);
const startCode = savedCode || `func mouse(int z, int y) {
    setFigureColor(Color::Red);
    rectangle(z, y, z+100, y+100);
    x = x + 10;
}

func keyboard(int key) {
    if (key == Key::Space) {
        setFigureColor(Color::Blue);
    } else {
      if (key == Key::A) {
          setFigureColor(Color::Black);
      } else {
          setFigureColor(Color::Yellow);
      }
    }
    x = x - 10;
}

global {
    int x = 320;
}

func main() {
   setLineColor(Color::Green);
   for i in (0..10000) {
      circle(x, 240, i % 100);
   }
   rectangle(0, 0, 100, 100);
}

`;

async function tryCompile(editor, src) {
  await initWasm();
  let idle_compiler = Compiler.new();
  const compilation_result = await idle_compiler.compile_code(src);   // Rust returns drawing commands (string)
   if (compilation_result.error_code != 0) {
    showError(editor.view, compilation_result);
    console.log(compilation_result.get_error_message() + " at " 
        + compilation_result.start_row + ":" + compilation_result.start_column
        + " - " + compilation_result.end_row + ":" + compilation_result.end_column);
  //   runBtn.disabled = false;
  //   return;
   } else {
    console.log("OK!");
  //   showOk(editor);
   }
}

let typingTimer = null;

const onTyping = EditorView.updateListener.of(update => { 
  if (update.docChanged) {
    update.view.dispatch(setDiagnostics(update.state, []));
    clearTimeout(typingTimer);

    // schedule a new one
    typingTimer = setTimeout(() => {
      const code = update.state.doc.toString();

      tryCompile(update, code);
      localStorage.setItem(STORAGE_KEY, editor.state.doc.toString());

    }, 1000); // 1000ms = 1 second pause
  }
});

const editor = new EditorView({
  state: EditorState.create({
    doc: startCode,
     extensions: [
    // A line number gutter
    lineNumbers(),
    // A gutter with code folding markers
     foldGutter(),
    // // Replace non-printable characters with placeholders
     highlightSpecialChars(),
    // // The undo history
     history(),
    // // Replace native cursor/selection with our own
     drawSelection(),
    // // Show a drop cursor when dragging over the editor
    // dropCursor(),
    // // Allow multiple cursors/selections
    // EditorState.allowMultipleSelections.of(true),
    // // Re-indent lines when typing specific input
     indentOnInput(),
    // // Highlight syntax with a default style
    //syntaxHighlighting(rustHighlighting),
    // // Highlight matching brackets near cursor
     bracketMatching(),
    // // Automatically close brackets
     closeBrackets(),
    // // Load the autocompletion system
     autocompletion(),
    // // Allow alt-drag to select rectangular regions
    // rectangularSelection(),
    // // Change the cursor to a crosshair when holding alt
    // crosshairCursor(),
    // // Style the current line specially
     highlightActiveLine(),
    // // Style the gutter for current line specially
     highlightActiveLineGutter(),
    quantaTheme,
    quantaLanguageSupport,
    //keymap.of([{key: "Tab", run: acceptCompletion}]),
    // Highlight text that matches the selected text
    //highlightSelectionMatches(),
    onTyping,
    insertFourSpaces,
    fourSpaceIndent,
    newlineSameIndent,
    keymap.of([
      // Closed-brackets aware backspace
      ...closeBracketsKeymap,
      // A large set of basic bindings
      ...defaultKeymap,
      // Redo/undo keys
      ...historyKeymap,
      // Code folding bindings
      ...foldKeymap,
      // Autocompletion keys
      ...completionKeymap,
      // Keys related to the linter system
      //...lintKeymap
    ])
  ]
    // extensions: [
    //   lineNumbers(),
    //   highlightActiveLine(),
    //   indentOnInput(),
    //   history(),
    //   autocompletion(),
    //   quanta(),
    //   
    //   oneDark
    // ]
  }),
  parent: document.getElementById("editor")
});

function clearErrors() {
  editor.dispatch(setDiagnostics(editor.state, []));
}



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

async function startExecution() {
  let res = runtime.execute();
  console.log("EXECUTION ENDED: " + res);
}

async function executeKey(key) {
  let res = runtime.execute_key(key);
  console.log("EXECUTION ENDED: " + res);
}

async function executeMouse(x, y) {
  let res = runtime.execute_mouse(x, y);
  console.log("EXECUTION ENDED: " + res);
}

// window.addEventListener('keydown', (e) => {
//   if (!runtime || !runtime.execute_key) return;

//   try {
//     console.log("Got key: " + e.key);
//     (async () => {runtime.execute_key(e.key);})(); // pass string like 'a', 'Enter', etc.
//   } catch (err) {
//     console.warn('Keyboard runtime error:', err);
//   }
// });

function doRun() {
  (async () => {
    try {
      cancelNow(false);
      isRunning = true;
      runBtn.disabled = true;
      setup();
      await initWasm();
      const src = editor.state.doc.toString();
      let compiler = Compiler.new();
      const compilation_result = await compiler.compile_code(src);   // Rust returns drawing commands (string)
      if (compilation_result.error_code != 0) {
        showError(editor, compilation_result);
        console.log(compilation_result.get_error_message() + " at " 
            + compilation_result.start_row + ":" + compilation_result.start_column
            + " - " + compilation_result.end_row + ":" + compilation_result.end_column);
        runBtn.disabled = false;
        return;
      } else {
        showOk(editor);
      }
      console.log("Compiling done");
      setRunningUI();
      runtime = compilation_result.get_runtime();
      startExecution();
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
            console.log("finish!");
            need_continue = false;
            break;
           }
        }
      }
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
  canvas.focus();
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

window.addEventListener('keydown', (e) => {
  if (!runtime) return;
  if (document.activeElement !== canvas) return;
  try {
    executeKey(e.key); // pass string like 'a', 'Enter', etc.
  } catch (err) {
    console.warn('Keyboard runtime error:', err);
  }
});

const resizer = document.getElementById('resizer');
const panes = document.querySelector('.panes');
let isDragging = false;

resizer.addEventListener('mousedown', (e) => {
  isDragging = true;
  document.body.style.cursor = 'col-resize';
});

window.addEventListener('mousemove', (e) => {
  if (!isDragging) return;
  const totalWidth = panes.getBoundingClientRect().width;
  const leftWidth = e.clientX;
  const rightWidth = totalWidth - leftWidth - 4; // 4 = resizer width
  panes.style.gridTemplateColumns = `${leftWidth}px 4px ${rightWidth}px`;
});

window.addEventListener('mouseup', () => {
  isDragging = false;
  document.body.style.cursor = '';
});

document.getElementById("canvas").addEventListener('click', (e) => {
  if (!runtime) return;

  const rect = canvas.getBoundingClientRect();
  const x = (e.clientX - rect.left) / rect.width * 1000;
  const y = (e.clientY - rect.top) /rect.height * 1000;

  try {
    const dpr = window.devicePixelRatio || 1;

  // Match canvas internal size to actual visible size * device pixel ratio
    executeMouse(x, y);
  } catch (err) {
    console.warn('Mouse runtime error:', err);
  }
});

// // Ctrl/Cmd+Enter
// addEventListener("keydown", (e) => {
//   const isMac = navigator.platform.toLowerCase().includes("mac");
//   if ((isMac ? e.metaKey : e.ctrlKey) && e.key === "Enter") {
//     e.preventDefault();
//     doRun();
//   }
// });

//resizeCanvasToDisplaySize();
editor.focus();

//const observer = new ResizeObserver(() => resizeCanvasToDisplaySize());
//observer.observe(canvas);
