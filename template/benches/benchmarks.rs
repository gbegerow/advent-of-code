use aoc_2024_22::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc_2024_22_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2_record() {
    aoc_2024_22_b(divan::black_box(INPUT));
}

#[divan::bench]
fn part2_all_sequences() {
    aoc_2024_22_b_all_sequences(divan::black_box(INPUT));
}
