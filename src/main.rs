use pollster::block_on;

fn main() {
    block_on(simulation::run());
}
