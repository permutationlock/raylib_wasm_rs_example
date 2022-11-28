# Cross-platform Rust apps using Raylib and Emscripten

The goal of this repository is to create a simple graphical Rust
application that can be compiled to target native platforms and web
browsers.

To cross-compile to wasm I use a
[Raylib](https://github.com/raysan5/raylib) library file compiled
with [emcc](https://emscripten.org), combined with a Rust wrapper
around the Raylib functionality needed in the project.

A custom Cargo config file is needed to pass emcc
build flags via the EMCC_CFLAGS environment variable.

Finally, in order to get applications to fill the browser window and
resize with it, we need to export an on_resize function from Rust to
JavaScript. This function is set as an event listener for the
Browser's window resize event.

## Dependencies

In order to compile to native Unix and/or Windows systems it is
assumed that you have the
[Raylib](https://github.com/raysan5/raylib) C library installed.
The web server provided to test wasm builds requires
[Go](https://go.dev).

## Building

Building native applications should work as normal:
```console
cargo build
```
To build for web:
```console
cargo build --target wasm32-unknown-emscripten
```
You can then test your web builds using the included Go server and shell
scripts.
