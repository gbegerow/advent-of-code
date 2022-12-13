use std::cmp::Ordering;
use std::error::{self, Error};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

/* input is pairs of nested lists of lists or int. Possible structure tree of lists.
Recursive solution? Seems legit JSON, can I exploit it? How does serde_json works?

Make an recursive enum and lets look how far we come
*/
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Package {
    List(Vec<Package>),
    Number(u32),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum PackageSyntaxError {
    Int(ParseIntError),
    SyntaxError,
}

impl fmt::Display for PackageSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PackageSyntaxError::SyntaxError => write!(f, "invalid syntax for package"),
            PackageSyntaxError::Int(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for PackageSyntaxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PackageSyntaxError::SyntaxError => None,
            PackageSyntaxError::Int(ref e) => Some(e),
        }
    }
}
impl From<ParseIntError> for PackageSyntaxError {
    fn from(e: ParseIntError) -> Self {
        PackageSyntaxError::Int(e)
    }
}

impl FromStr for Package {
    type Err = PackageSyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with("[") {

            // we have a list
            let stripped = s
                .strip_prefix("[").unwrap()
                .strip_suffix("]").unwrap();
            if stripped.is_empty() {return Ok(Package::List(Vec::new()))}

            let list = stripped
                .split(",") // split will splitt inside of lists :-(
                .map(|p| p.parse::<Package>().unwrap())
                .collect();
            Ok(Package::List(list))
        } else {
            // must be number only
            // if let Some((num_str, _)) = s.split_once(|c: char| !c.is_ascii_digit()) {
                Ok(Package::Number(s.parse()?))
            // } else {
            //     Err(PackageSyntaxError::SyntaxError)
            // }
        }
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            // numbers just compare
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            
            // convert Number to List in necesary
            (Self::Number(a), Self::List(_)) => Self::List(vec![Self::Number(*a)]).cmp(other),
            (Self::List(_), Self::Number(b)) => self.cmp(&Self::List(vec![Self::Number(*b)])),

            // lists compare item by item or whichever runs out first
            (Self::List(l), Self::List(r)) => {
                for (left, right) in l.iter().zip(r.iter()) {
                    match left.cmp(right) {
                        Ordering::Equal => { 
                            // no descision yet
                            continue;
                        }
                        other => {
                            // first non equal item decides
                            return other;
                        }
                    }
                }
                l.len().cmp(&r.len()) // which list has run out of items first
            }
        }
    }
}

fn compare_packages(left_str: &str, right_str: &str) -> Ordering {
    let left = left_str.parse::<Package>().unwrap();
    let right = right_str.parse::<Package>().unwrap();

    left.cmp(&right)
}

pub fn aoc_2022_13_a(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter_map(|pair| match pair.trim().split_once("\n") {
            Some((left, right)) => Some(compare_packages(left, right)),
            None => None,
        })
        .enumerate()
        .map(|(i, ord)| match ord {
            Ordering::Less | Ordering::Equal => i,
            Ordering::Greater => 0,
        })
        .sum()
}

pub fn aoc_2022_13_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Package;

    #[test]
    fn aoc_2022_13_a_example() {
        assert_eq!(super::aoc_2022_13_a(TEST_INPUT), 13);
    }

    #[test]
    fn aoc_2022_13_a() {
        assert_eq!(super::aoc_2022_13_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_13_b_example() {
        assert_eq!(super::aoc_2022_13_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_13_b() {
        assert_eq!(super::aoc_2022_13_b(include_str!("input.txt")), 0);
    }

    #[test]
    fn flat_list_should_less() {
        assert_eq!(
            super::compare_packages("[1,1,3,1,1]", "[1,1,5,1,1]"),
            Ordering::Less
        );
    }

    #[test]
    fn empty_lists_should_equal() {
        assert_eq!(super::compare_packages("[]", "[]"), 
                    Ordering::Equal)
    }
    
    #[test]
    fn parse_empty_list() {
        assert_eq!("[]".parse::<Package>(), Ok(Package::List(Vec::new())));
    }

    #[test]
    fn parse_flat_list() {
        assert_eq!("[1,2,3]".parse::<Package>(), 
        Ok(Package::List(vec![Package::Number(1),Package::Number(2),Package::Number(3),])));
    }

    #[test]
    fn parse_nested_list() {
        assert_eq!("[[1,2],3]".parse::<Package>(), 
        Ok(Package::List(vec![Package::List(vec![Package::Number(1),Package::Number(2)]),Package::Number(3),])));
    }

    const TEST_INPUT: &str = "[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";
}
