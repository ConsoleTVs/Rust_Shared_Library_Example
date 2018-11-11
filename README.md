# Rust shared library (.dll / .so) example with C program

This is a sample repository to show how to use a .dll / .so shared library generated from Rust.
This example focus only on how the generation, usage and linking works.

## How to use

### Using the Makefile

If you're using the GNU make utility, you may just run `make`

This should create the rust library and futher create the C executable.

### Using manually

#### Compile the rust library

```
rustc libcalc.rs --crate-type="dylib"
```

#### Compile and link the library with a C program

```
gcc main.c -o main.exe libcalc.dll
```
