// binary for perf tests etc.
use aoc_9999_99::{aoc_9999_99_b, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_9999_99_b(INPUT));
}
