build:
	(cd quanta_code_editor; make build)
run:
	(cd quanta_code_editor; make install; make build;)
	python3 -m http.server