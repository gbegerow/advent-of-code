// binary for perf tests etc.
use aoc_2025_04::{aoc_2025_04_a, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part A {}", aoc_2025_04_a(INPUT));
}
