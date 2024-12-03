use aoc_9999_99::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_9999_99_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_9999_99_b(divan::black_box(INPUT));
}
