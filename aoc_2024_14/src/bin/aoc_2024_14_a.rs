// binary for perf tests etc.
use aoc_2024_14::{aoc_2024_14_a, INPUT};
use glam::IVec2;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part A {}", aoc_2024_14_a(INPUT, IVec2::new(101, 103)));
}
