use aoc_2025_09::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_09_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_09_b(divan::black_box(INPUT));
}
