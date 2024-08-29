### Setting up

Everything is standard git/Rust, except that the git hooks are stored in the `.githooks` directory. Run `git config --local core.hooksPath .githooks` when setting up a new environment.

### Info

An experimental simulation, made so that I can play around with various technologies (for example, OpenGL).

As of this writing, the project is fairly bare-bones, but I might add more features to it later.

Features:
- Procedural terrain generation (2D)
- Simple map rendering with OpenGL
- Basic avatar, with movement and terrain collision

This project began as my final project for CSE 210 Programming With Classes. The assignment was to make something interesting, so I did, but I took it a little further than was required and continued to work on it after finishing the class. This included porting it from C# to Rust so that I could compile it to WASM (and because I wanted to learn Rust).
