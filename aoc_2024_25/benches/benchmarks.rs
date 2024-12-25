use aoc_2024_25::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_25_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2024_25_b(divan::black_box(INPUT));
}
