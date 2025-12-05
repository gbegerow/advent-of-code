// binary for perf tests etc.
use aoc_2025_05::{aoc_2025_05_b, INPUT};

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_2025_05_b(INPUT));
}
