use aoc_2025_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_10_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_10_b(divan::black_box(INPUT));
}
