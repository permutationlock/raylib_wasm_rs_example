# Cross-platform Rust apps using Raylib and Emscripten

The goal of this repository is to create a simple graphical Rust
application that can be compiled to target native platforms and web
browsers.

To achieve cross-compilation with wasm I use a
[Raylib](https://github.com/raysan5/raylib) library file compiled
with [emcc](https://emscripten.org) and a Rust wrapper around some of
Raylib's functionality.

Also needed is a custom config file to pass
emcc arguments via the EMCC_CFLAGS environment variable.

Finally, in order to get applications to fill the browser window and
resize with it, we need to export an on_resize call back from Rust to
JavaScript and set this function up as an event listener for the
Browser's window resize event.

## Dependencies

In order to compile to native Unix and/or Windows systems it is
assumed that you have the
[Raylib](https://github.com/raysan5/raylib) library installed.
The web server to test wasm builds requires [Go](https://go.dev).

## Building

Building native applications should work as normal. In order to do a
debug build for web simply run:
```console
cargo build --target wasm32-unknown-emscripten
```
You can then test your build using the included Go server and shell
script.
