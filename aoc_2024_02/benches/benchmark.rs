aoc_2024_02

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_a(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    part_b(divan::black_box(INPUT));
}
