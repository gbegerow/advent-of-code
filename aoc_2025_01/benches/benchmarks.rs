use aoc_2025_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_01_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_01_b(divan::black_box(INPUT));
}
