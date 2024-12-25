// binary for perf tests etc.
use aoc_2024_14::{aoc_2024_14_b, INPUT};
use glam::IVec2;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_2024_14_b(INPUT, IVec2::new(101, 103)));
}
