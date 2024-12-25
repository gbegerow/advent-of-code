use aoc_2024_18::*;
use glam::IVec2;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_18_a(divan::black_box(INPUT), IVec2::new(70, 70), 1024);
}

#[divan::bench]
fn part2() {
    aoc_2024_18_b(divan::black_box(INPUT), IVec2::new(70, 70), 1024);
}
