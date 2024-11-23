/* Find the task under https://adventofcode.com/2016/day/11
    Solution idea:
    A* in state space

    Representation:
    - Floor = Vec of Items, Facility = Vec of Floor
    - Elevator position

    Alternative representation:
    Store names/shorts in extra array immutable
    State is Vector of positions, alternating Chip Generator so Chip(3) = Index 2*3, Gen(0) = Index 0*2+1
    Position is 0..3 => 2 Bits => 1-4 Chips needs 4*2*2 Bits = u16 for complete state,  5-8 u32. 8 Chips with 16 Floors (4 Bit) => u64, 16 Chips => u128
    Assume 8 Chips in 16 Floors, not much more cost than 2 bit, but more flexible for b
      Debug output in hex, more compact. One item = one nibble
     can use bit ops for every test:
     - No gen on floor => state AND genmask XOR floormask == 0
     - No uncoupled chip on floor => shift down, State AND mask == floor && shiftdown, state AND mask == floor
    less memory, faster and easier tests. Display a little bit harder.
    set chip n to floor x => state OR= x shift right n*2 => easier move execute
    No movetype, just the new state
    Precalc:
        generator_mask => 0x0F repeat = 0x0F0F0F0F0F0F0F0F (or 0xF0?)
        generator_floor_mask =>  floor * generator_offset repeat
        item_mask = 0x0F

     chip 0 => Floor 0, gen 0 => Floor 1, chip 1 => Floor 0, gen 1 => Floor 2
        0x2010 => 0010000000010000
     chip 0 => Floor 1, gen 0 => Floor 1, chip 1 => Floor 0, gen 1 => Floor 2
        1 + 1 * 16 + 0 * 256 + 2 * 256* 16 => 0010000000010001

*/

use std::{collections::BinaryHeap, fmt, str::FromStr, string::ParseError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item {
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
            name: name.to_string(),
            short,
        }
    }
}

struct Facility {
    names: Vec<String>,
    shorts: Vec<String>,
}

type Distributionibution = u64;

/// Floor of chip and floor of generator
/// coupled = .chip == .gen
struct ItemPair {
    chip: u8,
    gen: u8,
}

impl ItemPair {
    fn new(pair: u8) -> Self {
        Self { chip: pair & 0x0F, gen:(pair >> 4) & 0x0F  }
    }
}

#[derive(Debug, Clone, Hash)]
struct State {
    elevator_at: usize,
    // Optimization idea: move floornumber back to item, use one continous Vec (size does not change) and sort. Keep track of indices of every pair.
    floors: Distributionibution,
    item_count: usize,
}

impl State {
    /// Get all valide moves from the current state on
    //#[inline(always)]
    fn get_valid_moves(&self) -> Vec<State> {
        // at least 1 Item, at most 2, elevator 1 up or down, no chip is allowed to be uncoupled and with another generator on the same floor
        // maybe SmallVec? no advantage

        // which directions might the elevator move
        let target_floors: [isize; 2] = match self.elevator_at {
            0 => [1, 0],
            1 | 2 => [-1, 1],
            3 => [-1, 0],
            _ => unreachable!("invalid floor"),
        };

       
        // generate all pairs  and all single items on the floor
        let mut iter = self.floors.iter().filter()
        let item_combinations = 
        // .collect();
        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));

        item_combinations
            .flat_map(|pair| {
                target_floors
                    .iter()
                    .filter(|f| **f != 0isize)
                    .map(move |f| 
                        State  {
                            elevator_at: self.elevator_at.saturating_add_signed(*f),
                            floors: self.floors.apply(pair),
                            item_count: self.item_count,
                        })
            })
            .filter(|m| m.is_valid())
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
        // top candidate for optimization

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
}


impl IntoIterator for State {
    type Item = ItemPair;
    type IntoIter = ItemIterator;
    
    fn into_iter(self) -> Self::IntoIter {
       ItemIterator {
            current: 0,
            distribution: self.floors,
            length: self.item_count,
        }
    }
}

pub struct ItemIterator {
    current: usize,
    distribution: Distribution,

}

impl Iterator for ItemIterator{
    type Item = ItemPair;

    fn next(&mut self) -> Option<u8>{
        if self.current < self.length {
            let pair = ItemPair::new(self.distribution >> 8*self.current);                
            self.current += 1;
            Some(pair)
        } else {None}
    }
}


impl fmt::Display for State {
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

impl FromStr for State {
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

        Ok(State {
            floors,
            elevator_at: 0,
        })
    }
}

pub fn aoc_2016_11_a(input: &str) -> usize {
    let f: State = input.parse().expect("invalid input");

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
        let fac: State = TEST_INPUT.parse().unwrap();
        assert!(fac.is_valid_floor(floor.iter()) == exepected);
    }

    #[rstest]
    #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "hydrogen")]}, 0, true)]
    #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "lithium")]}, 0, false)]
    #[case(Move{elevator_to:0, items:vec![Item::new(Itemtype::Generator, "hydrogen")]}, 1, false)]
    fn move_should_be(#[case] move_to: Move, #[case] elevator_at: usize, #[case] exepected: bool) {
        let mut fac: State = TEST_INPUT.parse().unwrap();
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
