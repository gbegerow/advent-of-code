use std::fmt;
use std::{cmp::Ordering, fmt::Debug};
use std::str::FromStr;
use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{digit1},
    error::{context, VerboseError},
    multi::{separated_list0},
    sequence::delimited,
    IResult, Finish
};
use regex::Regex;

/* input is pairs of nested lists of lists or int. Possible structure tree of lists.
Recursive solution? Seems legit JSON, can I exploit it? How does serde_json works?

Make an recursive enum and lets look how far we come
*/
#[derive(Debug, PartialEq, Eq, Clone)]
enum Package {
    List(Vec<Package>),
    Number(u32),
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Package::Number(i) => std::fmt::Display::fmt(i, f),
            Package::List(l) => {
                write!(f, "[" )?;
                for p in l{
                    std::fmt::Display::fmt(p, f)?
                }
                write!(f, "]" )
            },
        }
    }
}


type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn number(input:&str)-> Res<&str, Package>
{
    context("number", digit1)(input)
        .map(|(next_input, res)|{
            (next_input, Package::Number(res.parse().unwrap()))
        })
}
fn list(input:&str) -> Res<&str, Package>
{
    context("list",
        delimited(tag("["),
            separated_list0(tag(","), 
                alt((list, number))
            ), 
            tag("]"))
        )(input)
        .map(|(next_input,  res)|{
            (next_input, Package::List(res))
        })
}

// #[allow(dead_code)]
impl FromStr for Package {
    type Err = VerboseError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match list(s.trim()).finish() {
            Ok((_rest, package)) => Ok(package),
            Err(e ) => Err(VerboseError {
                errors: e.errors.iter().map(|(s, k)| (s.to_string(), k.clone())).collect(),
            }),
        }
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            // numbers just compare
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            
            // convert Number to List if necesary
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

// derived PartialOrd relies on order of declaration in enum which is just garbage in this case
impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn compare_packages(left_str: &str, right_str: &str) -> Ordering {
    let left = left_str.parse::<Package>().unwrap();
    let right = right_str.parse::<Package>().unwrap();

    let order = left.cmp(&right);

    // println!("cmp({}, {}) => {:?}", left, right, order);
    order
}

pub fn aoc_2022_13_a(input: &str) -> usize {
    let empty_line = Regex::new(r"(?m)^\s*$").unwrap();
   
    let pairs: Vec<_> = empty_line.split(input.trim())
        .enumerate()
        .collect();

    // println!("{:?}", pairs);    
    let orderings: Vec<_> = pairs.iter()
        .map(|(i,pair)| match pair.trim().split_once("\n") {
            Some((left, right)) => (i, compare_packages(left, right)),
            None => (i, Ordering::Greater),
        })
        .collect();

    orderings.iter()    
        .map(|(i, ord)| match ord {
            Ordering::Less | Ordering::Equal => *i+1,
            Ordering::Greater => 0,
        })
        .sum()
}

pub fn aoc_2022_13_b(input: &str) -> usize {
    let divider: Vec<Package> = vec!["[[2]]","[[6]]"].iter()
    .filter_map(|l|  l.parse::<Package>().ok())
    .collect();    

    let mut orderings: Vec<Package> = input.lines()
        .filter_map(|l|  l.parse::<Package>().ok())
        .collect();

    orderings.append(&mut divider.clone());    
    orderings.sort_unstable();

    // for (i, o) in orderings.iter().enumerate() {
    //     println!("{}: {:?}", i+1, o);
    // }

   divider.iter()
        .filter_map(|d| orderings.iter().position(|o| o == d)) 
        .map(|p| p + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use regex::Regex;

    use crate::Package;

    #[test]
    fn aoc_2022_13_a_example() {
        assert_eq!(super::aoc_2022_13_a(TEST_INPUT), 13);
    }

    #[test]
    fn aoc_2022_13_a() {
        assert_eq!(super::aoc_2022_13_a(include_str!("input.txt")), 5506);
    }

    #[test]
    fn aoc_2022_13_b_example() {
        assert_eq!(super::aoc_2022_13_b(TEST_INPUT), 140);
    }

    #[test]
    fn aoc_2022_13_b() {
        assert_eq!(super::aoc_2022_13_b(include_str!("input.txt")), 0);
    }

    // ------------------------------ Unit tests ------------------------------

    #[test]
    fn empty_lists_should_equal() {
        assert_eq!(super::compare_packages("[]", "[]"), 
                    Ordering::Equal)
    }
    
    
    #[test]
    fn flat_list_should_less() {
        assert_eq!(
            super::compare_packages("[1,1,3,1,1]", "[1,1,5,1,1]"),
            Ordering::Less
        );
    }

    #[test]
    fn nested_list_should_less() {
        assert_eq!(
            super::compare_packages(" [[1],[2,3,4]]","[[1],4]"),
            Ordering::Less
        );
    }

    #[test]
    fn nested_list_should_less_2() {
        assert_eq!(super::compare_packages("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[[2]]"), Ordering::Less);
    }
    
    #[test]
    fn nested_list_should_less_3() {
        assert_eq!(super::compare_packages("[[1],[2,3,4]]", "[1,[2,[3,[4,[5,6,7]]]],8,9]"), Ordering::Less);
    }
    
    #[test]
    fn nested_list_should_less_4() {
        assert_eq!(super::compare_packages("[[1],[2,3,4]]", "[[2]]"), Ordering::Less);
    }
    
    #[test]
    fn nested_list_should_less_5() {
        assert_eq!(super::compare_packages("[[8,7,6]]", "[3]"), Ordering::Greater);
    }
    
    #[test]
    fn nested_list_should_less_6() {
        assert_eq!(super::compare_packages("[[6]]", "[3]"), Ordering::Greater);
    }
    
    #[test]
    fn parse_digit() {
        assert_eq!(super::number("1"), Ok(("", Package::Number(1))));
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
    
    #[test]
    fn parse_nested_list2() {
        assert_eq!("[[1],[2,3,4]]".parse::<Package>(), 
        Ok(Package::List(vec![
            Package::List(vec![Package::Number(1)]),
            Package::List(vec![Package::Number(2),Package::Number(3),Package::Number(4),])
            ]))
        );
    }

    #[test]

    fn split_by_empty_lines() {
        let empty_line = Regex::new(r"(?m)^\s*$").unwrap();
        assert_eq!(empty_line.split(TEST_INPUT).collect::<Vec<_>>().len(), 8)
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


// old try
/*

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


*/