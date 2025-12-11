// binary for perf tests etc.
use aoc_2025_10::{aoc_2025_10_b, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_2025_10_b(INPUT));
}
