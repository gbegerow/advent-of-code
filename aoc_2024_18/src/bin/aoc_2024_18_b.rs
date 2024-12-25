// binary for perf tests etc.
use aoc_2024_18::{aoc_2024_18_b, INPUT};
use glam::IVec2;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    println!("Part B {}", aoc_2024_18_b(INPUT, IVec2::new(70, 70), 1024));
}
