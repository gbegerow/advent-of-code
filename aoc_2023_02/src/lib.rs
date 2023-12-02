// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/02
    Solution idea:

*/

pub fn aoc_2023_02_a(input: &str) -> i32 {
    let mut possisble_games = 0;
    for line in input.trim().lines() {
        let (game, rolls) = line.split_once(":").unwrap();
        let game_num: i32 = game
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .last()
            .unwrap();

        if rolls
            .split([';', ',']) // you can split over multiple separators
            .filter_map(|d| d.trim().split_once(" "))
            .map(|(n_str, c)| (n_str.trim().parse::<i32>().unwrap(), c))
            .all(|(n, color)| match color {
                "red" if n > 12 => false,
                "green" if n > 13 => false,
                "blue" if n > 14 => false,
                _ => true,
            })
        {
            // game is possisble
            possisble_games += game_num;
        }
    }

    possisble_games
}

pub fn aoc_2023_02_b(input: &str) -> i32 {
    let mut power = 0;
    for line in input.trim().lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (_game, rolls) = line.split_once(":").unwrap();

        let max = rolls
            .split(";")
            .flat_map(|r| r.split(","))
            .filter_map(|d| d.trim().split_once(" "))
            .map(|(n_str, c)| (n_str.trim().parse::<i32>().unwrap(), c))
            .fold((0,0,0), |max, (n, color)| match color { 
                "red" if n > max.0 => (n, max.1, max.2),
                "green" if n > max.1 => (max.0, n, max.2),
                "blue" if n > max.2 =>  (max.0, max.1, n),
                _ => max,
            });
        power += max.0 * max.1 * max.2;
    }

    power
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_02_a_example() {
        assert_eq!(super::aoc_2023_02_a(TEST_INPUT), 8);
    }

    #[test]
    fn aoc_2023_02_a() {
        assert_eq!(super::aoc_2023_02_a(INPUT), 2076);
    }

    #[test]
    fn aoc_2023_02_b_example() {
        assert_eq!(super::aoc_2023_02_b(TEST_INPUT), 2286);
    }

    #[test]
    fn aoc_2023_02_b() {
        assert_eq!(super::aoc_2023_02_b(INPUT), 70950);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
}
