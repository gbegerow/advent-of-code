use aoc_2024_22::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// 100 runs * 1h 50 ~ 7.5 days. DO NOT RUN!!!!!
#[divan::bench(sample_count = 5)]
fn part1() {
    //todo!("Do not run any of the brute force as benchmark with full input!!");
    // aoc_2024_22_a(divan::black_box(INPUT));
    aoc_2024_22_a(divan::black_box(TEST_INPUT));
}

#[divan::bench(sample_count = 5)]
fn part2() {
    // todo!("Do not run any of the brute force as benchmark with full input!!");
    // aoc_2024_22_b(divan::black_box(INPUT));
    aoc_2024_22_b(divan::black_box(TEST_INPUT));
}

const TEST_INPUT: &str = "1
10
100
2024";
