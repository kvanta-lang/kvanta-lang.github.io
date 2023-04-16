# Quanta

Installation instructions:

1. git clone this ;)

2. Tree-sitter
 - Install tree-sitter (inside grammar directory): https://tree-sitter.github.io/tree-sitter/creating-parsers#installation
  - Compile grammar from grammar.js (add all generated files to .gitignore) "tree-sitter generate"
  - Check grammar via "tree-sitter parse text.txt"

3. Rust & WASM
 - Install WebAssembly for Rust tools: https://rustwasm.github.io/docs/book/game-of-life/setup.html
- In root dir, compile Rust module to WASM: "wasm-pack build --release --target web"

4. Launch index.html on localhost: in root directory "py -m http.server"

5. Go to 127.0.0.1:8000 and enjoy)
