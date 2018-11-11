all: libcalc.dll
	gcc main.c -o main.exe libcalc.dll

libcalc.dll:
	rustc libcalc.rs --crate-type="dylib"
