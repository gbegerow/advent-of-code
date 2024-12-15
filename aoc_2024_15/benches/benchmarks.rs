use aoc_2024_15::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_15_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2024_15_b(divan::black_box(INPUT));
}
