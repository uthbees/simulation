#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// Use .expect() with a helpful message instead.
#![warn(clippy::unwrap_used)]

mod window;

fn main() {
    window::run();
}
