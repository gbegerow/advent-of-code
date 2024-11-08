use std::{fmt, str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Itemtype {
    Chip,
    Generator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    ty: Itemtype,
    name: String,
    short: String,
}

impl Item {
    fn new(ty: Itemtype, name: &str) -> Self {
        let first_char = name
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .next()
            .expect("should have at least one letter");
        let short = match ty {
            Itemtype::Chip => format!("{first_char}M"),
            Itemtype::Generator => format!("{first_char}G"),
        };
        Self {
            ty,
            name: name.to_string(),
            short,
        }
    }
}

struct Move {
    elevator_to: usize,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
struct Facility {
    elevator_at: usize,
    floors: Vec<Vec<Item>>,
}

impl Facility {
    /// Get all valide moves from the current state on
    fn get_valid_moves(&self) -> Vec<Move> {
        // at least 1 Item, at most 2, elevator 1 up or down, no chip is allowed to be uncoupled and with another generator on the same floor
        // maybe SmallVec? no advantage

        // which directions might the elevator move
        let target_floors: [isize; 2] = match self.elevator_at {
            0 => [1, 0],
            1 | 2 => [-1, 1],
            3 => [-1, 0],
            _ => unreachable!("invalid floor"),
        };

        let items = &self.floors[self.elevator_at];
        // generate all pairs  and all single items on the floor
        let item_combinations = items.iter().flat_map(|i| {
            items.iter().map(move |j| {
                if i == j {
                    vec![i.clone()]
                } else {
                    vec![i.clone(), j.clone()]
                }
            })
        });
        // .collect();
        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));

        item_combinations
            .flat_map(|pair| {
                target_floors
                    .iter()
                    .filter(|f| **f != 0isize)
                    .map(move |f| Move {
                        elevator_to: self.elevator_at.saturating_add_signed(*f),
                        items: pair.clone(),
                    })
            })
            .filter(|m| self.is_valid_move(m))
            .collect::<Vec<_>>()
    }

    /// Does the move produce a valid state?
    fn is_valid_move(&self, m: &Move) -> bool {
        // is from floor valid without the moved items
        let from = self.floors[self.elevator_at]
            .iter()
            .filter(|i| !m.items.contains(i));
        if !self.is_valid_floor(from) {
            return false;
        }

        // is the target floor valid with added items
        let to = self.floors[m.elevator_to].iter().chain(m.items.iter());
        self.is_valid_floor(to)
    }

    /// Is floor valid? - no uncoupled chips on same floor as any generator
    fn is_valid_floor<'a>(&self, floor: impl Iterator<Item = &'a Item> + Clone) -> bool {
        // no generator -> no danger -> valid
        if !floor.clone().any(|i| i.ty == Itemtype::Generator) {
            println!("No Generator");
            return true;
        }

        // let coupled = floor
        //     .clone()
        //     .filter(|i| i.ty == Itemtype::Chip)
        //     .flat_map(|i| {
        //         match floor
        //             .clone()
        //             .find(|g| g.ty == Itemtype::Generator && g.name == i.name)
        //         {
        //             Some(g) => Some((i, g)),
        //             None => None,
        //         }
        //     })
        //     .collect::<Vec<_>>();
        // println!("Coupled: {:?}", coupled);

        // no uncoupled microchip -> valid
        // small vectors, linear search is faster than HashMap
        // maybe do this only once on insert?
        let all_coupled = floor.clone().filter(|i| i.ty == Itemtype::Chip).all(|i| {
            floor
                .clone()
                .find(|g| g.ty == Itemtype::Generator && g.name == i.name)
                .is_some()
        });

        all_coupled
    }

    /// Is Facility in final state? (all chips and generators on last floor)
    fn is_final(&self) -> bool {
        // only last floor may have items
        self.floors[..self.floors.len() - 1]
            .iter()
            .all(|f| f.len() == 0)
    }

    /// Heuristic distance to final state
    fn distance(&self) -> usize {
        self.floors[..self.floors.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let floor_factor = self.floors.len() - i;
                floor_factor * floor_factor * f.len()
            })
            .sum()
    }

    pub fn a_star(&mut self) -> Vec<Move> {
        todo!("A*");
    }
}

/*
// A* in state space
// Every state is a complete facility + move to get from previous form
// theoretically we could just reconstruct state it every time from moves
pub fn a_star(&mut self, start:&Coordinate) -> Option<Vec<Coordinate>>
{
    let capacity = self.width*self.height;

    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    let mut open =BinaryHeap::new();
    open.push(start.clone());

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from = HashMap::with_capacity(capacity);
    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score:HashMap<Coordinate, usize> = HashMap::with_capacity(capacity);
    g_score.insert(start.clone(), 0);
    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    let mut f_score:HashMap<Coordinate, usize> = HashMap::with_capacity(capacity);
    f_score.insert(start.clone(), self.distance(&start, &self.end));

    // println!("{}x{}={} start: {} end: {}",
    //     self.width, self.height, capacity, start, self.end);

    while let Some((current, _)) = open.pop() {
        // print!("{current}");
        // println!("current: {} open: {} came_from: {} g_score: {} f_score: {}",
        //     current, open.len(), came_from.len(), g_score.len(), f_score.len()
        // );

        // reached goal?
        if current == self.end {
            return Some(Self::reconstruct_path(came_from, current))
        }

        for neighbour in self.get_connected_neighbours(&current){
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            let d = self.distance(&current, &neighbour); // d must be 1 (same height) or 2 (highher/lower), maybe always 1?
            let tentative_g_score = g_score[&current] + d;
            if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                // This path to neighbor is better than any previous one. Record it!
                let h = self.distance(&self.start,&neighbour);
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + h);
                if open.iter().all(|n| *n.0 != neighbour) {
                    open.push(neighbour, usize::MAX - h ); // priority queue uses highest prio, we want lowest distance
                }
            }
        }

    }

    // Open set is empty but goal was never reached
    None
}
    */

impl fmt::Display for Facility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (floor, items) in self.floors.iter().enumerate().rev() {
            writeln!(
                f,
                "L{} {} {}",
                floor + 1,
                if self.elevator_at == floor {
                    "=>"
                } else {
                    "  "
                },
                items
                    .iter()
                    .map(|s| s.short.as_str())
                    .collect::<Vec<&str>>()
                    .join(" . ")
            )?;
        }
        Ok(())
    }
}

impl FromStr for Facility {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let floors = input
            .trim()
            .lines()
            .map(|line| {
                // skip "The first floor contains"
                let words: Vec<_> = line.split([' ', '-', ',', '.']).skip(4).collect();
                // println!("{words:?}");
                let mut items = Vec::new();
                for i in 0..words.len() {
                    match words[i] {
                        // a hydrogen-compatible microchip
                        "microchip" => items.push(Item::new(Itemtype::Chip, words[i - 2])),

                        // a hydrogen generator
                        "generator" => items.push(Item::new(Itemtype::Generator, words[i - 1])),
                        _ => (),
                    }
                }
                items
            })
            .collect();

        Ok(Facility {
            floors,
            elevator_at: 0,
        })
    }
}

pub fn aoc_2016_11_a(input: &str) -> usize {
    let f: Facility = input.parse().expect("invalid input");

    println!("{f}");
    0
}

pub fn aoc_2016_11_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 11)]
    fn aoc_2016_11_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_11_a(input), exepected);
    }

    #[rstest]
    #[case(vec![Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Chip, "lithium")], true)]
    #[case(vec![Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Generator, "hydrogen")], true)]
    #[case(vec![Item::new(Itemtype::Chip, "lithium"), Item::new(Itemtype::Generator, "hydrogen")], false)]
    #[case(vec![Item::new(Itemtype::Chip, "lithium"), Item::new(Itemtype::Generator, "hydrogen")], false)]
    #[case(vec![Item::new(Itemtype::Generator, "lithium"), Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Generator, "hydrogen")], true)]
    fn floor_should(#[case] floor: Vec<Item>, #[case] exepected: bool) {
        let fac: Facility = TEST_INPUT.parse().unwrap();
        assert!(fac.is_valid_floor(floor.iter()) == exepected);
    }

    #[rstest]
    #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "hydrogen")]}, 0, true)]
    #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "lithium")]}, 0, false)]
    #[case(Move{elevator_to:0, items:vec![Item::new(Itemtype::Generator, "hydrogen")]}, 1, false)]
    fn move_should_be(#[case] move_to: Move, #[case] elevator_at: usize, #[case] exepected: bool) {
        let mut fac: Facility = TEST_INPUT.parse().unwrap();
        fac.elevator_at = elevator_at;
        assert!(fac.is_valid_move(&move_to) == exepected)
    }

    #[test]
    fn aoc_2016_11_a() {
        assert_eq!(super::aoc_2016_11_a(INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2016_11_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_11_b(input), exepected);
    }

    #[test]
    fn aoc_2016_11_b() {
        assert_eq!(super::aoc_2016_11_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
}
