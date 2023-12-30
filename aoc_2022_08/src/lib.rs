use std::collections::HashSet;

pub fn aoc_2022_08_a(input: &str) -> usize {
    let (map, w, h) = parse_heightmap(input);
    let mut visible = HashSet::new();

    for y in 0..h {
        for x in 0..w {
            if is_visible(&map, w, h, x, y) {
                visible.insert((x, y));
            }
        }
    }

    visible.len()
}

pub fn aoc_2022_08_b(input: &str) -> usize {
    let (map, w, h) = parse_heightmap(input);
    let mut max_score = 0;

    for y in 0..h {
        for x in 0..w {
            let score = scenic_score(&map, w, h, x, y);

            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn scenic_score(map: &Vec<u32>, w: usize, h: usize, x: usize, y: usize) -> usize {
    let cell = get(map, w, x, y);

    // outer edge has always score 0
    if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
        return 0;
    }

    let mut score = 1;

    // count inclusive the first one breaking the condition (take_while would not count the first tree blocking sight)

    // score to left
    let mut run = 0;
    for t in (w * y..w * y + x).rev() {
        run += 1;
        if map[t] >= cell {
            break;
        }
    }
    score *= run;

    // score to right
    run = 0;
    for t in w * y + x + 1..w * (y + 1) {
        run += 1;
        if map[t] >= cell {
            break;
        }
    }
    score *= run;

    // score up
    run = 0;
    for t in (0..y).rev() {
        run += 1;
        if map[w * t + x] >= cell {
            break;
        }
    }
    score *= run;

    // score down
    run = 0;
    for t in y + 1..h {
        run += 1;
        if map[w * t + x] >= cell {
            break;
        }
    }
    score *= run;

    score
}

fn is_visible(map: &Vec<u32>, w: usize, h: usize, x: usize, y: usize) -> bool {
    // let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // deltas axis aligned
    let cell = get(map, w, x, y);

    // outer edge is always visible
    let mut visible = x == 0 || y == 0 || x == w - 1 || y == h - 1;

    if !visible {
        // compound assignment |= should shortcut if visible is already true? maybe not in debug build?
        // visible from left or right?
        visible = visible || map[w * y..w * y + x].iter().all(|t| t < &cell);
        visible = visible || map[w * y + x + 1..w * (y + 1)].iter().all(|t| t < &cell);

        // visible from up or down?
        visible = visible || (0..y).all(|py| map[w * py + x] < cell);
        visible = visible || (y + 1..h).all(|py| map[w * py + x] < cell);
    }

    visible
}

fn get(map: &Vec<u32>, w: usize, x: usize, y: usize) -> u32 {
    let index = w * y + x;
    map[index]
}

fn parse_heightmap(input: &str) -> (Vec<u32>, usize, usize) {
    let mut heightmap = Vec::with_capacity(100 * 100);
    let mut width = 0;
    let mut height = 0;

    for line in input.lines() {
        let mut t: Vec<u32> = line.trim().chars().filter_map(|c| c.to_digit(10)).collect();
        height += 1;
        if width < t.len() {
            width = t.len();
        }
        heightmap.append(&mut t);
    }
    (heightmap, width, height)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_08_a_example() {
        assert_eq!(super::aoc_2022_08_a(TEST_INPUT), 21);
    }

    #[test]
    fn aoc_2022_08_a() {
        assert_eq!(super::aoc_2022_08_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_08_b_example() {
        assert_eq!(super::aoc_2022_08_b(TEST_INPUT), 8);
    }

    #[test]
    fn aoc_2022_08_b() {
        assert_eq!(super::aoc_2022_08_b(include_str!("input.txt")), 209880);
    }

    const TEST_INPUT: &str = "   30373
    25512
    65332
    33549
    35390";
}
