install:
	npm i 
	(cd quanta; npm i)

build:
	(cd quanta; npm run build)
	npm run prepare
	node_modules/.bin/rollup editor.mjs -f esm -o editor.bundle.js -n "editor" \
  -p @rollup/plugin-node-resolve

build-code-editor:
	node_modules/.bin/rollup editor.mjs -f esm -o editor.bundle.js -n "editor" \
  -p @rollup/plugin-node-resolve

up:	
	python3 -m http.server 

run:
	make install
	make build
	make up
