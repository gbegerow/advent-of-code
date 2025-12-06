use aoc_2025_06::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_06_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_06_b(divan::black_box(INPUT));
}
