use aoc_2024_20::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_20_a(divan::black_box(INPUT), 100);
}

#[divan::bench]
fn part2() {
    aoc_2024_20_b(divan::black_box(INPUT), 100);
}
