.PHONY: run
run:
	gcc -o c_pkg main.c -L ../rust_pkg/target/debug -l rust_pkg
	LD_LIBRARY_PATH=../rust_pkg/target/debug ./c_pkg
