import {EditorView, basicSetup} from "codemirror"
import {quanta} from "./dist/index.js"
import {keymap} from "@codemirror/view"
import {acceptCompletion} from "@codemirror/autocomplete"

async function init_code_editor(field_checker) {
  let sync_val = localStorage.getItem("code");

  console.log("Hello from Quanta Code Editor!");

  function createCodeEditor(initialValue) {
    if (initialValue === undefined) {
      initialValue = sync_val?.length > 0 ? sync_val : "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
    }
    return new EditorView({
      extensions: [
        basicSetup,
        quanta(),
        keymap.of([{key: "Tab", run: acceptCompletion}]),
        EditorView.updateListener.of(
          function(e) {
            sync_val = e.state.doc.toString();
            localStorage.setItem("code", sync_val);
          }
        ),
        EditorView.theme({
          "&": {height: "420px"},
          ".cm-scroller": {overflow: "auto"}
        }),
      ],
      doc: initialValue,
      parent: document.getElementById("code"),
    });
  }

  function download(filename, text) {
    var element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename);
  
    element.style.display = 'none';
    document.body.appendChild(element);
  
    element.click();
  
    document.body.removeChild(element);
  }

  let editor = createCodeEditor();

  function run_code() {
    console.log("Running code...")
    let text = editor.state.doc.toString().trim();
    let canvas = document.getElementById("drawing");
    const ctx = canvas.getContext("2d");
    let canvas_state = field_checker(text).field;
    let cell_width = canvas.width / canvas_state.length;
    let cell_height = canvas.height / canvas_state.length;
    for (var i = 0; i < canvas_state.length; i++) {
      for (var j = 0; j < canvas_state[i].length; j++) {
        let r  = canvas_state[i][j] >> 16 & 255;
        let g  = canvas_state[i][j] >> 8 & 255;
        let b  = canvas_state[i][j] & 255; 
        if (i === 10 && j === 10 ) {
          ctx.fillStyle = "rgba(0,0,0,1)";
        } else {
        ctx.fillStyle = "rgba("+r+","+g+","+b+",1)";
        }
        ctx.fillRect( i, j, cell_width, cell_height);
      }
    }
  }

  function saveCode() {
    console.log("Saving code...");
    download('code.txt', editor.state.doc.toString());
  }

  document.getElementById("fileWithCode").onchange = function(evt) {
    console.log("Loading code...");
    if(!window.FileReader) return; // Browser is not compatible
    console.log("Loading code...");
    var reader = new FileReader();

    reader.onload = function(evt) {
        if(evt.target.readyState != 2) return;
        if(evt.target.error) {
            alert('Error while reading file');
            return;
        }

        const filecontent = evt.target.result;
        editor.dom.parentNode.removeChild(editor.dom);

        editor = createCodeEditor(filecontent);
    };

    reader.readAsText(evt.target.files[0]);
};
  

  document.getElementById("runButton").addEventListener("click", run_code);
  document.getElementById("saveButton").addEventListener("click", saveCode);
  document.getElementById("saveImageButton").addEventListener('click', function (e) {
    const link = document.createElement('a');
    link.download = 'download.png';
    let canvas = document.getElementById("drawing");
    link.href = canvas.toDataURL();
    link.click();
    link.delete;
  });
}

export {init_code_editor};