// binary for perf tests etc.
use aoc_9999_99::{aoc_9999_99_a, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part A {}", aoc_9999_99_a(INPUT));
}
