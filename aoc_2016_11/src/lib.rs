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

use std::{
    collections::{binary_heap::Iter, BinaryHeap},
    fmt,
    ops::Index,
    str::FromStr,
    string::ParseError,
};

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct Item {
//     name: String,
//     short: String,
// }

// impl Item {
//     fn new( name: &str) -> Self {
//         let first_char = name
//             .chars()
//             .map(|c| c.to_ascii_uppercase())
//             .next()
//             .expect("should have at least one letter");
//         Self {
//             name: name.to_string(),
//             short: first_char,
//         }
//     }
// }

struct Facility {
    names: Vec<String>,
    shorts: Vec<String>,
}

type Distribution = u64;
// get: ((self >> index * BUCKET_SIZE) & BUCKET_MASK) as u8
// set:

/// How many chip/generator pairs have we capacity for
const CAPACITY: usize = 8;
/// How many bits we have per position
const BUCKET_SIZE: usize = 4;
/// Mask for isolating a single position
const BUCKET_MASK: u8 = 0xF;
/// How many bits per pair
const PAIR_SIZE: usize = 2 * BUCKET_SIZE;
/// Mask for isolating a pair
const PAIR_MASK: u8 = 0xFF;

// indexer not possible as we cannot build a referene to a value
// allow for  let chip4 = floors[5]
// impl Index<usize> for Distribution {
//     type Output = u8;

//     fn index(&self, index: usize) -> &Self::Output {
//         ((self >> index * 4) & 0xF) as u8
//     }
// }

/// Floor of chip and floor of generator
/// coupled = .chip == .gen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ItemPair {
    chip: u8,
    gen: u8,
    index: usize,
}

impl ItemPair {
    fn new(index: usize, pair: u8) -> Self {
        Self {
            index,
            chip: pair & BUCKET_MASK,
            gen: (pair >> BUCKET_SIZE) & BUCKET_MASK,
        }
    }

    #[inline]
    fn is_coupled(&self) -> bool {
        self.chip == self.gen
    }
}

struct IterItemPair {
    pos: usize,
    pair_length: usize,
    state: Distribution,
}
/// iterate over chip/generator pairs
impl Iterator for IterItemPair {
    type Item = ItemPair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.pair_length {
            return None;
        }

        let item_pair = ItemPair::new(self.pos, (self.state >> self.pos * PAIR_SIZE) as u8);
        self.pos += 1;

        Some(item_pair)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FloorItem {
    item: u8,
    index: usize,
}

impl FloorItem {
    fn new(index: usize, item: u8) -> Self {
        Self { index, item }
    }
}

struct IterFloorItem {
    pos: usize,
    length: usize,
    state: Distribution,
}
impl Iterator for IterFloorItem {
    type Item = FloorItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.length {
            return None;
        }

        let item = ((self.state >> self.pos * BUCKET_SIZE) & BUCKET_MASK as Distribution) as u8;
        let floor_item = FloorItem::new(self.pos, item);

        self.pos += 1;

        Some(floor_item)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    elevator_at: usize,
    // Optimization idea: move floornumber back to item, use one continous Vec (size does not change) and sort. Keep track of indices of every pair.
    floors: Distribution,
    item_count: usize,
}

impl State {
    fn new(elevator_at: usize, floors: Distribution, item_count: usize) -> Self {
        Self {
            elevator_at,
            floors,
            item_count,
        }
    }

    fn iter(&self) -> IterFloorItem {
        IterFloorItem {
            pos: 0,
            length: self.item_count,
            state: self.floors,
        }
    }

    fn pairs(&self) -> IterItemPair {
        IterItemPair {
            pos: 0,
            pair_length: self.item_count / 2,
            state: self.floors,
        }
    }

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
        let items_on_floor = self
            .iter()
            .filter(|i| i.item == (self.elevator_at as u8))
            .collect::<Vec<_>>();

        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let item_combinations = items_on_floor.iter().flat_map(|i| {
            items_on_floor.iter().map(move |j| {
                if i.index == j.index {
                    vec![i.index]
                } else {
                    vec![i.index, j.index]
                }
            })
        });

        item_combinations
            .flat_map(|combination| {
                target_floors
                    .iter()
                    .filter(|f| **f != 0isize)
                    .map(move |f| self.apply_move(f, &combination))
            })
            .filter(|m| m.is_valid())
            .collect::<Vec<_>>()
    }

    fn apply_move(&self, move_by: &isize, items: &Vec<usize>) -> Self {
        let new_pos = self.elevator_at.saturating_add_signed(*move_by);
        let mut state = self.floors;

        for index in items {
            // set the indexed nibble to the value of the floor
            let mask = (BUCKET_MASK as Distribution) << (index * BUCKET_SIZE);
            let pos =
                (new_pos as Distribution & BUCKET_MASK as Distribution) << index * BUCKET_SIZE;

            state = (state & !mask) | pos;
            // println!(
            //     "index: {index} mask: {mask:06x}  new_pos: {new_pos} pos: {pos:06x} state: {state:06x}"
            // )
        }

        State {
            elevator_at: new_pos,
            floors: state,
            item_count: self.item_count,
        }
    }

    /// Is floor valid? - no uncoupled chips on same floor as any generator
    #[inline]
    fn is_valid(&self) -> bool {
        // (was) top candidate for optimization

        // valid => filter all pairs coupled (gen == chip).
        // For Rest no chip is allowed to be on same floor as any gen
        //  => intersection of generators and chips must be empty

        // now test should be relative cheap
        // can we prevent allocation of hashsets here and reuse it? Optimization for later
        // hashing and hashset is overkill. Use u64 as binary set limited to bitcount floors.
        // Set a bit corresponding to floor for every item, c AND g should be 0
        // No bit in common = sets disjoint = chips and generators on different floors
        let mut c = 0;
        let mut g = 0;
        for ItemPair { chip, gen, .. } in self.pairs().filter(|p| !p.is_coupled()) {
            c |= 1 << chip;
            g |= 1 << gen;
        }

        (c & g) == 0
    }

    /// Is Facility in final state? (all chips and generators on last floor)
    #[inline]
    fn is_final(&self) -> bool {
        // only last floor may have items
        // final => all chip nibbles == 4 and all gen == 4
        (self.floors & 0x44_44_44_44_44_44_44_44u64) == self.floors
    }

    /// Heuristic distance to final state
    fn distance(&self) -> usize {
        // distance to final floor has most influence (cubic) but also distance between chip and genarator (linear or quadratic)
        // sum of (max_floor - floor)³ + (|chip - gen|)
        todo!("distance")
    }
}

// impl fmt::Display for State {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (floor, items) in self.iter().enumerate().rev() {
//             writeln!(
//                 f,
//                 "L{} {} {}",
//                 floor + 1,
//                 if self.elevator_at == floor {
//                     "=>"
//                 } else {
//                     "  "
//                 },
//                 items
//                     .iter()
//                     .map(|s| s.short.as_str())
//                     .collect::<Vec<&str>>()
//                     .join(" . ")
//             )?;
//         }
//         Ok(())
//     }
// }

// impl FromStr for State {
//     type Err = ParseError;

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         let floors = input
//             .trim()
//             .lines()
//             .map(|line| {
//                 // skip "The first floor contains"
//                 let words: Vec<_> = line.split([' ', '-', ',', '.']).skip(4).collect();
//                 // println!("{words:?}");
//                 let mut items : Distribution = Vec::new();
//                 for i in 0..words.len() {
//                     match words[i] {
//                         // a hydrogen-compatible microchip
//                         "microchip" => items.push(Item::new(Itemtype::Chip, words[i - 2])),

//                         // a hydrogen generator
//                         "generator" => items.push(Item::new(Itemtype::Generator, words[i - 1])),
//                         _ => (),
//                     }
//                 }
//                 items
//             })
//             .collect();

//         Ok(State {
//             floors,
//             elevator_at: 0,
//         })
//     }
// }

pub fn aoc_2016_11_a(_input: &str) -> usize {
    // let f: State = input.parse().expect("invalid input");

    // println!("{f}");
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
    #[case(0x44_33_22_11_00, vec![ItemPair::new(0, 0x00), ItemPair::new(1, 0x11), ItemPair::new(2, 0x22), ItemPair::new(3, 0x33), ItemPair::new(4,0x44)])]
    #[case(0x40_30_20_10_01, vec![ItemPair::new(0, 0x01), ItemPair::new(1, 0x10), ItemPair::new(2, 0x20), ItemPair::new(3, 0x30), ItemPair::new(4,0x40)])]
    fn pair_iterator_should(#[case] floors: Distribution, #[case] expected: Vec<ItemPair>) {
        let sut = State::new(4, floors, 6);

        let pairs = sut.pairs().collect::<Vec<_>>();
        assert_eq!(&pairs[..], &expected[..])
    }

    #[rstest]
    #[case(0x44_33_22_11_00, vec![FloorItem::new(0, 0), FloorItem::new(1, 0), FloorItem::new(2, 1), FloorItem::new(3, 1), FloorItem::new(4,2), FloorItem::new(5,2), FloorItem::new(6,3), FloorItem::new(7,3), FloorItem::new(8,4), FloorItem::new(9,4)])]
    #[case(0x98_76_54_32_10, vec![FloorItem::new(0, 0), FloorItem::new(1, 1), FloorItem::new(2, 2), FloorItem::new(3, 3), FloorItem::new(4,4), FloorItem::new(5,5), FloorItem::new(6,6), FloorItem::new(7,7), FloorItem::new(8,8), FloorItem::new(9,9)])]
    fn item_iterator_should(#[case] floors: Distribution, #[case] expected: Vec<FloorItem>) {
        let sut = State::new(4, floors, 6);

        let items = sut.iter().collect::<Vec<_>>();
        assert_eq!(&items[..], &expected[..])
    }

    #[rstest]
    #[case(0x22_11_00, true)] // all coupled
    #[case(0x44_44_44, true)] // all coupled
    #[case(0x22_10_10, true)] // chips and generators on different floors
    #[case(0x22_01_10, false)] // uncoupled chips on floor with other generator
    fn state_valid_should(#[case] floors: Distribution, #[case] expected: bool) {
        let sut = State::new(4, floors, 6);

        assert_eq!(sut.is_valid(), expected);
    }

    #[rstest]
    #[case(0x22_11_00, false)] // all coupled
    #[case(0x44_44_44, true)] // all coupled
    fn state_final_should(#[case] floors: Distribution, #[case] expected: bool) {
        let sut = State::new(4, floors, 6);

        assert_eq!(sut.is_final(), expected);
    }

    #[rstest]
    #[case(0x32_32, -1, vec![0,2], State::new(1, 0x31_31, 4) )]
    #[case(0x32_32,  1, vec![0,2], State::new(3, 0x33_33, 4) )]
    #[case(0x32_32,  1, vec![2], State::new(3, 0x33_32, 4) )]
    fn state_apply_move_should(
        #[case] floors: Distribution,
        #[case] move_by: isize,
        #[case] items: Vec<usize>,
        #[case] expected: State,
    ) {
        let sut = State::new(2, floors, 4);

        let post_move = sut.apply_move(&move_by, &items);

        assert_eq!(post_move, expected);
    }

    // #[rstest]
    // #[case(vec![Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Chip, "lithium")], true)]
    // #[case(vec![Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Generator, "hydrogen")], true)]
    // #[case(vec![Item::new(Itemtype::Chip, "lithium"), Item::new(Itemtype::Generator, "hydrogen")], false)]
    // #[case(vec![Item::new(Itemtype::Chip, "lithium"), Item::new(Itemtype::Generator, "hydrogen")], false)]
    // #[case(vec![Item::new(Itemtype::Generator, "lithium"), Item::new(Itemtype::Chip, "hydrogen"), Item::new(Itemtype::Generator, "hydrogen")], true)]
    // fn floor_should(#[case] floor: Vec<Item>, #[case] exepected: bool) {
    //     let fac: State = TEST_INPUT.parse().unwrap();
    //     assert!(fac.is_valid_floor(floor.iter()) == exepected);
    // }

    // #[rstest]
    // #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "hydrogen")]}, 0, true)]
    // #[case(Move{elevator_to:1, items:vec![Item::new(Itemtype::Chip, "lithium")]}, 0, false)]
    // #[case(Move{elevator_to:0, items:vec![Item::new(Itemtype::Generator, "hydrogen")]}, 1, false)]
    // fn move_should_be(#[case] move_to: Move, #[case] elevator_at: usize, #[case] exepected: bool) {
    //     let mut fac: State = TEST_INPUT.parse().unwrap();
    //     fac.elevator_at = elevator_at;
    //     assert!(fac.is_valid_move(&move_to) == exepected)
    // }

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
