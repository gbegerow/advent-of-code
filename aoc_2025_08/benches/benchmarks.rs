use aoc_2025_08::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_08_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_08_b(divan::black_box(INPUT));
}
