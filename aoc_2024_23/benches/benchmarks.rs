use aoc_2024_23::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_23_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2024_23_b(divan::black_box(INPUT));
}
