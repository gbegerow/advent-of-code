use aoc_2024_12::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_12_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2024_12_b(divan::black_box(INPUT));
}
