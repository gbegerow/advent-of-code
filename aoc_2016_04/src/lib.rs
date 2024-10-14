use regex::Regex;
use std::str::FromStr;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/04
    Solution idea:

*/
#[derive(Debug)]
struct Room {
    name_enc: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn new(name_enc: &str, sector_id: &str, checksum: &str) -> Self {
        let name_enc = name_enc.to_string(); //.replace("-", "");
        let sector_id = sector_id.parse().expect("Sector ID should be numeric");
        let checksum = checksum.to_string();

        Self {
            name_enc,
            sector_id,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let calculated_checksum = calc_checksum(self.name_enc.as_str());

        self.checksum == calculated_checksum
    }

    fn decrypt(&self) -> String {
        //format!("{:?}", self)
        self.name_enc
            .chars()
            .map(|c| match c {
                a if a.is_ascii_lowercase() => rotate(a, self.sector_id),
                '-' => ' ',
                _ => unreachable!("invalid character"),
            })
            .collect::<String>()
    }
}

fn rotate(c: char, rotate: u32) -> char {
    assert!(c.is_ascii_lowercase());

    let ord = c as u8 - b'a';
    let rotated = (ord as u32 + rotate) % 26;

    println!("c: {c} ord: {ord} rotated: {rotated} by {rotate}");
    char::from(rotated as u8 + b'a')
}

#[derive(Debug, Clone, Copy)]
struct RoomParseError;

impl FromStr for Room {
    type Err = RoomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: Regex =
            Regex::new(r"^\s*(?P<name_enc>[a-z-]+)-(?P<sector_id>\d+)\[(?P<checksum>\w+)\]\s*$")
                .unwrap();
        if let Some(cap) = re.captures(s) {
            Ok(Room::new(
                &cap["name_enc"],
                &cap["sector_id"],
                &cap["checksum"],
            ))
        } else {
            Err(RoomParseError)
        }
    }
}

fn calc_checksum(s: &str) -> String {
    let mut histogram = [0u32; 26];

    for c in s.bytes() {
        if c.is_ascii_lowercase() {
            histogram[c.wrapping_sub(b'a') as usize] += 1;
        }
    }

    // take the 5 most used characters. break a tie using alphabetic order.
    let mut alphabet = (0usize..26usize).collect::<Vec<_>>();
    alphabet.sort_by(|a, b| histogram[*b].cmp(&histogram[*a]).then(a.cmp(b))); // highest hist first, lowes chr first

    // println!(
    //     "string: '{}'\nhistogram: {}\nalphabet: {}",
    //     s,
    //     histogram
    //         .iter()
    //         .enumerate()
    //         .map(|(i, h)| format!("{}:{}, ", char::from(i as u8 + b'a'), h))
    //         .collect::<String>(),
    //     alphabet
    //         .iter()
    //         .map(|c| char::from(*c as u8 + b'a'))
    //         .collect::<String>()
    // );

    alphabet
        .iter()
        .take(5)
        .map(|c| char::from(*c as u8 + b'a'))
        .collect::<String>()
}

pub fn aoc_2016_04_a(input: &str) -> usize {
    input
        .trim()
        .lines()
        .flat_map(|l| l.parse::<Room>())
        .filter(|room| room.is_real())
        .map(|room| room.sector_id as usize)
        .sum()
}

pub fn aoc_2016_04_b(input: &str) -> usize {
    let rooms = input
        .trim()
        .lines()
        .flat_map(|l| l.parse::<Room>())
        .filter(|room| room.is_real())
        .collect::<Vec<_>>();

    for x in rooms
        .iter()
        .map(|r| format!("{} => {}", &r.name_enc, r.decrypt()))
    {
        println!("{}", x);
    }

    if let Some(found) = rooms.iter().find(|room| room.decrypt().contains("north"))
    // "north pole objects")
    {
        println!("{found:?}");
        found.sector_id.try_into().unwrap()
    } else {
        99999
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aaaaabbbzyx", "abxyz")]
    #[case("abcdefgh", "abcde")]
    #[case("notarealroom", "oarel")]
    #[case("totallyrealroom", "loart")]
    fn calc_checksum_should(#[case] input: &str, #[case] exepected: &str) {
        let checksum = calc_checksum(input);
        assert_eq!(checksum, exepected);
    }

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", true)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]", true)]
    #[case("not-a-real-room-404[oarel]", true)]
    #[case("totally-real-room-200[decoy]", false)]
    #[case("qzmt-zixmtkozy-ivhz-343[zimth]", true)]
    fn room_is_real_should(#[case] input: &str, #[case] exepected: bool) {
        let room: Room = input.parse().unwrap();
        assert_eq!(room.is_real(), exepected);
    }

    #[rstest]
    #[case(
        "aaaaa-bbb-z-y-x-123[abxyz]
    a-b-c-d-e-f-g-h-987[abcde]
    not-a-real-room-404[oarel]
    totally-real-room-200[decoy]",
        1514
    )]
    fn aoc_2016_04_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_04_a(input), exepected);
    }

    #[test]
    fn aoc_2016_04_a() {
        assert_eq!(super::aoc_2016_04_a(INPUT), 278221);
    }

    #[rstest]
    #[case("qzmt-zixmtkozy-ivhz-343[zimth]", "very encrypted name")]
    #[case("north-pole-objects-26[oetbc]", "north pole objects")]
    #[case("abcdef-1[abcde]", "bcdefg")]
    fn decrypt_should(#[case] input: &str, #[case] exepected: &str) {
        let room: Room = input.parse().unwrap();
        assert_eq!(room.decrypt(), exepected);
    }

    #[test]
    fn aoc_2016_04_b_example() {
        assert_eq!(
            super::aoc_2016_04_b("qzmt-zixmtkozy-ivhz-343[zimth]"),
            99999
        );
    }

    #[test]
    fn aoc_2016_04_b() {
        assert_eq!(super::aoc_2016_04_b(INPUT), 267);
    }

    const INPUT: &str = include_str!("input.txt");
}
