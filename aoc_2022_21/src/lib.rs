use std::{collections::HashMap, hash::Hash};

use regex::Regex;
// #[allow(dead_code, unused_variables)]

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BinOp {
    lhs: String,
    rhs: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Ops {
    Plus(BinOp),
    Minus(BinOp),
    Multiply(BinOp),
    Divide(BinOp),
    Match(BinOp),
    Eval(String),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Expression {
    Number(i64),
    Operation(Ops),
}

fn parse(input: &str, is_part_b: bool) -> HashMap<String, Expression> {
    let rx = Regex::new(
        r"(?mx)
    \s* (?P<name>\w+):     #  name of node
    (?:  # either operation lhs op rhs
       \s* (?P<lhs>\w+) # name of lhs  
       \s* (?P<op>\+|-|\*|/|=) # operation
       \s* (?P<rhs>\w+) # name of rhs
   | # or number
      \s* (?P<num>[-\d]+)
    )
    ",
    )
    .unwrap();

    let mut tree = HashMap::new();
    for caps in rx.captures_iter(input) {
        if caps.name("op").is_some() {
            tree.insert(
                caps["name"].to_string(),
                Expression::Operation({
                    let bin = BinOp {
                        lhs: caps["lhs"].to_string(),
                        rhs: caps["rhs"].to_string(),
                    };
                    match &caps["op"] {
                        "+" => Ops::Plus(bin),
                        "-" => Ops::Minus(bin),
                        "*" => Ops::Multiply(bin),
                        "/" => Ops::Divide(bin),
                        "=" => Ops::Match(bin),
                        _ => panic!("valid operator expected"),
                    }
                }),
            );
        } else if caps.name("num").is_some() {
            tree.insert(
                caps["name"].to_string(),
                Expression::Number(caps["num"].parse::<i64>().unwrap()),
            );
        }
    }

    if is_part_b {
        // rewrite tree
        tree.insert(
            "root".to_string(),
            match &tree["root"] {
                Expression::Operation(Ops::Plus(b)) => Expression::Operation(Ops::Match(b.clone())),
                Expression::Operation(Ops::Minus(b)) => {
                    Expression::Operation(Ops::Match(b.clone()))
                }
                Expression::Operation(Ops::Multiply(b)) => {
                    Expression::Operation(Ops::Match(b.clone()))
                }
                Expression::Operation(Ops::Divide(b)) => {
                    Expression::Operation(Ops::Match(b.clone()))
                }
                _ => panic!("root must be binary operation"),
            },
        );

        tree.insert("humn".to_string(), Expression::Number(0));
    }
    tree
}

fn eval(root: &str, ast: &HashMap<String, Expression>) -> i64 {
    match &ast[root] {
        Expression::Number(i) => i.clone(),
        Expression::Operation(op) => match op {
            Ops::Plus(BinOp { lhs: l, rhs: r }) => eval(&l, ast) + eval(&r, ast),
            Ops::Minus(BinOp { lhs: l, rhs: r }) => eval(&l, ast) - eval(&r, ast),
            Ops::Multiply(BinOp { lhs: l, rhs: r }) => eval(&l, ast) * eval(&r, ast),
            Ops::Divide(BinOp { lhs: l, rhs: r }) => eval(&l, ast) / eval(&r, ast),
            Ops::Match(BinOp { lhs: l, rhs: r }) => {
                let lhs = eval(&l, ast);
                let rhs = eval(&r, ast);

                -1 * (lhs - rhs) // for the moment just the diff between sides
            },
            Ops::Eval(subtree) => eval(&subtree, ast),
        },
    }
}


fn find_ops_on(ast: &HashMap<String, Expression>, node : &str) -> Vec<String> {
    let humn_ops = ast
        .into_iter()
        .filter(|&(_, exp)| {
            if let Some( binop) = match exp {
                Expression::Operation(Ops::Plus(b)) => Some(b),
                Expression::Operation(Ops::Minus(b)) => Some(b),
                Expression::Operation(Ops::Multiply(b)) => Some(b),
                Expression::Operation(Ops::Divide(b)) => Some(b),
                _ => None,
            } {
                binop.lhs == node || binop.rhs == node
            } else {
                false
            }
        }).map(|e| e.0)
        .cloned()
        .collect::<Vec<_>>();
    humn_ops
}

fn invert(e: Expression) -> Expression {
    match e {
        Expression::Operation(Ops::Plus(b)) => Expression::Operation(Ops::Minus(b.clone())),
        Expression::Operation(Ops::Minus(b)) => Expression::Operation(Ops::Plus(b.clone())),
        Expression::Operation(Ops::Multiply(b)) => Expression::Operation(Ops::Divide(b.clone())),
        Expression::Operation(Ops::Divide(b)) => Expression::Operation(Ops::Multiply(b.clone())),
        _ => e.clone()
    }
}

fn invert_path(ast: &HashMap<String, Expression>, start: &str) -> HashMap<String, Expression> {
// invert path: evaluate in inverse order inverse operation, eval other operands

    let mut path = HashMap::new();
    let mut node_name = start.to_string();
    while node_name != "root"  {
        let inv = invert(ast[&node_name].clone());
        path.insert(node_name.to_string(), inv);

        let mut ops = find_ops_on(ast, &node_name);
        debug_assert_eq!(ops.len(), 1, "node to invert is used multiple times");
        let parent = ops.swap_remove(0);
        node_name = parent;   
    }

    path.insert("root".to_string(), Expression::Operation(Ops::Eval(node_name.to_string())));
    path
}

// fn depth(ast:&HashMap<String, Expression>){
//     let max_depth = 1;

// }

pub fn aoc_2022_21_a(input: &str) -> i64 {
    let ast = parse(input, false);

    // println!("{:?}", ast);
    eval("root", &ast)
}

pub fn aoc_2022_21_b(input: &str) -> i64 {
    let ast = parse(input, true);
    println!("{:?}", ast);

    let m = eval("root", &ast);
    let mut inverter = invert_path(&ast, "humn");
    _ = inverter.insert("humn".to_string(), Expression::Number(m));
    eval("root", &inverter)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{BinOp, Expression, Ops, find_ops_on, invert_path};

    #[test]
    fn aoc_2022_21_a_example() {
        assert_eq!(super::aoc_2022_21_a(TEST_INPUT), 152);
    }

    #[test]
    fn aoc_2022_21_a() {
        assert_eq!(super::aoc_2022_21_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_21_b_example() {
        assert_eq!(super::aoc_2022_21_b(TEST_INPUT), 301);
    }

    #[test]
    fn aoc_2022_21_b() {
        assert_eq!(super::aoc_2022_21_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "
    root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32
    ";

    // -------------- Unit tests -------------------

    #[test]
    fn parse_should_op() {
        assert_eq!(
            super::parse("root: pppw + sjmn", false),
            HashMap::from([(
                "root".to_string(),
                Expression::Operation(Ops::Plus(BinOp {
                    lhs: "pppw".to_string(),
                    rhs: "sjmn".to_string()
                }))
            )])
        );
    }

    #[test]
    fn parse_should_num() {
        assert_eq!(
            super::parse("humn: -10", false),
            HashMap::from([("humn".to_string(), Expression::Number(-10))])
        );
    }

    #[test]
    fn humn_is_used_only_once() {
        let ast = super::parse(include_str!("input.txt"), true);

        let humn_ops = find_ops_on(&ast, "humn");

        println!("{:?}", humn_ops);
        assert_eq!(humn_ops.len(), 1);
    }

    #[test]
    fn test_invert_path(){
        let ast = super::parse(TEST_INPUT, true);
        assert_eq!(invert_path(&ast, "humn"), 
        HashMap::from([(
            "root".to_string(),
            Expression::Operation(Ops::Plus(BinOp {
                lhs: "pppw".to_string(),
                rhs: "sjmn".to_string()
            }))
        )])
    
    );
    }
}
