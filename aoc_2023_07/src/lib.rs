
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

    Part b: If there is a joker, the joker can be used rank up the hand.
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    rank: usize, // A has a rank higher than all other labels,
    label: char,
}

impl Card {
    fn new(label: char, joker: bool) -> Self {
        let rank = if joker {
            // joker has rank 0 as indivual card
            "J23456789TQKA"
        } else {
            "23456789TJQKA"
        }
        .find(label)
        .expect("invald card label");

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
    joker: bool,
}

use HandType::*;

impl Hand {
    fn new(s: &str, joker: bool) -> Self {
        let (s_cards, s_bid) = s.trim().split_once(" ").expect("expected cards bid");
        let bid = s_bid.parse().expect("invalid bid");
        let cards: [Card; 5] = s_cards
            .chars()
            .take(5)
            .map(|c| Card::new(c, joker))
            .collect::<Vec<_>>()
            .try_into()
            .expect("invalid cards");

        let hand_type = Self::get_handtype(&cards, joker);
        Self {
            cards,
            hand_type,
            bid,
            joker,
        }
    }

    fn count_cards(cards: &[Card; 5]) -> [u8; 13] {
        let mut counts = [0; 13];
        for card in cards {
            counts[card.rank as usize] += 1;
        }
        counts
    }

    fn get_handtype(cards: &[Card; 5], joker: bool) -> HandType {
        // could be expressed a lot shorter, might even more readable
        let counts = Self::count_cards(cards);
        let mut hand_type = HighCard;
        let mut pairs = 0;
        for count in counts
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if joker && i==0  { None } else { Some(c) })
        {
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

        if joker {
            let joker_count = counts[0];
            // calculate hand type += joker_count except for full house
            match (joker_count, hand_type) {
                (1, HighCard) => hand_type = OnePair,
                (2, HighCard) => hand_type = ThreeOfAKind,
                (3, HighCard) => hand_type = FourOfAKind,
                (4, HighCard) => hand_type = FiveOfAKind,
                (5, HighCard) => hand_type = FiveOfAKind,                
                (1, OnePair) => hand_type = ThreeOfAKind,
                (2, OnePair) => hand_type = FourOfAKind,
                (3, OnePair) => hand_type = FiveOfAKind,
                (1, TwoPairs) => hand_type = FullHouse,
                (1, ThreeOfAKind) => hand_type = FourOfAKind,
                (2, ThreeOfAKind) => hand_type = FiveOfAKind,
                (1, FourOfAKind) => hand_type = FiveOfAKind,
                _ => (),
            }
        }    

        hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.hand_type, self.cards).partial_cmp(&(other.hand_type, other.cards))
    }
}

fn score(input: &str, joker: bool) -> usize {
    parse(input, joker)
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

fn parse(input: &str, joker: bool) -> Vec<Hand> {
    let mut hands_down: Vec<_> = input.trim().lines().map(|s| Hand::new(s, joker)).collect();
    hands_down.sort();
    hands_down
}

pub fn aoc_2023_07_a(input: &str) -> usize {
    score(input, false)
}

pub fn aoc_2023_07_b(input: &str) -> usize {
    score(input, true)
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
        assert_eq!(super::aoc_2023_07_a(INPUT), 252295678);
    }

    #[test]
    fn aoc_2023_07_b_example() {
        assert_eq!(super::aoc_2023_07_b(TEST_INPUT), 5905);
    }

    #[test]
    fn aoc_2023_07_b() {
        assert_eq!(super::aoc_2023_07_b(INPUT), 250577259);
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
    fn handtype_should_be_correct_a(#[case] cards: &str, #[case] expected: crate::HandType) {
        let sut = super::Hand::new(cards, false);
        assert_eq!(sut.hand_type, expected);
    }

    #[rstest]
    #[case("23456 213", HighCard)]
    #[case("J3456 213", OnePair)]
    #[case("2345J 213", OnePair)]    
    #[case("32T3K 765", OnePair)]
    #[case("T55J5 684", FourOfAKind)]
    #[case("KK677 28", TwoPairs)]
    #[case("KTJJT 220", FourOfAKind)]
    #[case("QQQJA 483", FourOfAKind)]
    #[case("JJJJJ 001", FiveOfAKind)]
    #[case("QQQQJ 001", FiveOfAKind)]
    #[case("JQQQQ 001", FiveOfAKind)]
    #[case("QQQQ2 483", FourOfAKind)]
    fn handtype_should_be_correct_b(#[case] cards: &str, #[case] expected: crate::HandType) {
        let sut = super::Hand::new(cards, true);
        assert_eq!(sut.hand_type, expected);
    }

    #[test]
    fn hands_should_be_sorted() {
        let sut = super::parse(TEST_INPUT, true);
        assert_eq!(sut, 
            vec![
                super::Hand::new("32T3K 765", true),
                super::Hand::new("KK677 28", true),
                super::Hand::new("T55J5 684", true),
                super::Hand::new("QQQJA 483", true),
                super::Hand::new("KTJJT 220", true),
            ]
        );
    }

    #[test]
    fn hands_leading_joker_should_be_sorted() {
        let sut = super::parse("AAAAA 001\nJJJJJ 002\n22222 003", true);
        assert_eq!(sut, 
            vec![
                super::Hand::new("JJJJJ 002", true),
                super::Hand::new("22222 003", true),
                super::Hand::new("AAAAA 001", true),
            ]
        );
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
}
