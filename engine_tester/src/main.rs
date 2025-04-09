use polyhedra::run;
use pollster;

fn main() {
    pollster::block_on(run()); // Functionality for async
}