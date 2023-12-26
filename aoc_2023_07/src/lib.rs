use std::fmt;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/07
    Solution idea:
    Count all cards with same rank.
    If there are 5 cards with the same rank => five_of_a_kind.
    If there are 4 cards with the same rank => four_of_a_kind.
    If there are 3 cards with the same rank and 2 cards with the same rank => full_house.
    If there are 3 cards with the same rank => three_of_a_kind.
    If there are 2 cards with the same rank and 2 other cards with the same rank => two_pairs.
    If there are 2 cards with the same rank => one_pair.
    If there are no cards with the same rank => high_card.
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    rank: usize, // A has a rank higher than all other labels,
    label: char,
}

impl Card {
    fn new(label: char) -> Self {
        let rank = "23456789TJQKA".find(label).expect("invald card label");

        Self { rank, label }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

use HandType::*;

impl Hand {
    fn new(s: &str) -> Self {
        let (s_cards, s_bid) = s.trim().split_once(" ").expect("expected cards bid");
        let bid = s_bid.parse().expect("invalid bid");
        let cards: [Card; 5] = s_cards
            .chars()
            .take(5)
            .map(|c| Card::new(c))
            .collect::<Vec<_>>()
            .try_into()
            .expect("invalid cards");

        let hand_type = Self::get_handtype(&cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn count_cards(cards: &[Card; 5]) -> [u8; 13] {
        let mut counts = [0; 13];
        for card in cards {
            counts[card.rank as usize] += 1;
        }
        counts
    }

    fn get_handtype(cards: &[Card; 5]) -> HandType {
        let counts = Self::count_cards(cards);
        let mut hand_type = HighCard;
        let mut pairs = 0;
        for count in counts.iter() {
            match count {
                2 => pairs += 1,
                3 => hand_type = ThreeOfAKind,
                4 => hand_type = FourOfAKind,
                5 => hand_type = FiveOfAKind,
                _ => (),
            }
        }

        if hand_type == ThreeOfAKind && pairs == 1 {
            hand_type = FullHouse;
        } else if hand_type == HighCard && pairs == 2 {
            hand_type = TwoPairs;
        } else if hand_type == HighCard && pairs == 1 {
            hand_type = OnePair;
        }
        hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.hand_type, self.cards).partial_cmp(&(other.hand_type, other.cards))
    }
}


pub fn aoc_2023_07_a(input: &str) -> usize {
    let mut hands_down: Vec<_> = input.trim().lines().map(|s| Hand::new(s)).collect();
    hands_down.sort();

    hands_down
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

pub fn aoc_2023_07_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::HandType::*;
    use rstest::rstest;

    #[test]
    fn aoc_2023_07_a_example() {
        assert_eq!(super::aoc_2023_07_a(TEST_INPUT), 6440);
    }

    #[test]
    fn aoc_2023_07_a() {
        assert_eq!(super::aoc_2023_07_a(INPUT), 0);
    }

    #[test]
    fn aoc_2023_07_b_example() {
        assert_eq!(super::aoc_2023_07_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_07_b() {
        assert_eq!(super::aoc_2023_07_b(INPUT), 0);
    }

    #[rstest]
    #[case("23456 213", HighCard)]
    #[case("32T3K 765", OnePair)]
    #[case("T55J5 684", ThreeOfAKind)]
    #[case("KK677 28", TwoPairs)]
    #[case("KTJJT 220", TwoPairs)]
    #[case("QQQJA 483", ThreeOfAKind)]
    #[case("JJJJJ 001", FiveOfAKind)]
    #[case("QQQQ2 483", FourOfAKind)]
    fn handtype_should_be_correct(#[case] cards: &str, #[case] expected: crate::HandType) {
        let sut = super::Hand::new(cards);
        assert_eq!(sut.hand_type, expected);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
}
