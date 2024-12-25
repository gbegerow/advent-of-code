use aoc_2024_14::*;
use glam::IVec2;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_14_a(divan::black_box(INPUT), IVec2::new(101, 103));
}

#[divan::bench]
fn part2() {
    aoc_2024_14_b(divan::black_box(INPUT), IVec2::new(101, 103));
}
