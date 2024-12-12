// binary for perf tests etc.
use aoc_2024_12::{aoc_2024_12_a, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part A {}", aoc_2024_12_a(INPUT));
}
