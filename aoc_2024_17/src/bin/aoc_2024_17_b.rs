// binary for perf tests etc.
use aoc_2024_17::{aoc_2024_17_b, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_2024_17_b(INPUT));
}