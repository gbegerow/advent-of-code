use aoc_2024_08::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_08_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2024_08_b(divan::black_box(INPUT));
}
