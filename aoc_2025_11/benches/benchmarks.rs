use aoc_2025_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2025_11_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    aoc_2025_11_b(divan::black_box(INPUT));
}
