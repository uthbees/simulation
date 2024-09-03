### Setting up

Everything is standard git/Rust, except that the git hooks are stored in the `.githooks` directory. Run `git config --local core.hooksPath .githooks` when setting up a new environment.

To build the web version, install `wasm-pack` (with `cargo install wasm-pack`) and run `wasm-pack build --target web --debug`. The updated page will then be accessible from `index.html`, although you will need to use a web server to access it so that it can load the wasm file from `pkg`. (Your IDE can probably do this for you.)

### Info

An experimental simulation, made so that I can play around with various technologies (for example, OpenGL).

As of this writing, the project is fairly bare-bones, but I might add more features to it later.

Features:
- Procedural 2D terrain generation
- Simple map rendering with a low-level graphics API provided by wgpu
- Available on both desktop and WASM (mostly thanks to winit and wgpu doing all the platform interfacing stuff for me)

This project began as my final project for CSE 210 Programming With Classes. The assignment was to make something interesting, so I did, but I took it a little further than was required and continued to work on it after finishing the class. This included porting it from C# to Rust so that I could compile it to WASM (and because I wanted to learn Rust).
