use aoc_2025_02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_02_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_02_b(divan::black_box(INPUT));
}
