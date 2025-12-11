// binary for perf tests etc.
use aoc_2025_09::{aoc_2025_09_a, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part A {}", aoc_2025_09_a(INPUT));
}
