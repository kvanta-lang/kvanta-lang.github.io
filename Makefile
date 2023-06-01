build:
	(cd quanta_code_editor; make build)
run:
	(cd quanta_code_editor; make install; make build;)
	python3 -m http.server

light-run:
	(cd quanta_code_editor; make build-code-editor)
	python3 -m http.server