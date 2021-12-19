use std::{collections::{HashSet, HashMap}, str::FromStr, time::Instant};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} μs", start.elapsed().as_micros())
}

fn part1() {
    let input = include_str!("input.test2.txt");

    let snp: SnailFishPair = input.lines().nth(0).unwrap().parse().unwrap();
    let mut accum: Vec<(i8, Item)> = Vec::new();
    flatten(snp, &mut accum, 0);

    for line in input.lines().skip(1) {
        let next: SnailFishPair = line.parse().unwrap();
        let mut next_accum: Vec<(i8, Item)> = Vec::new();
        flatten(next, &mut next_accum, 0);

        println!("  {:?}", accum);
        println!("+ {:?}", next_accum);
        
        let combined = combine(&accum, &next_accum);
        let result = step(&combined);
        accum = result;
        println!("= {:?}", accum);
    }

    println!("result: {:?}", accum);
    println!("result: {:?}", magnitude(&accum, 3));
}

fn part2() {
    let _input = include_str!("input.txt");
}

#[test]
fn test_parse() {
    let mut pair: SnailFishPair = "[1,2]".parse().unwrap();
    assert_eq!(
        SnailFishPair::new(Item::RegularNumber(1), Item::RegularNumber(2)),
        pair
    );

    pair = "[[1,2],3]".parse().unwrap();
    assert_eq!(
        SnailFishPair::new(
            Item::Pair(SnailFishPair::new(
                Item::RegularNumber(1),
                Item::RegularNumber(2)
            )),
            Item::RegularNumber(3)
        ),
        pair
    );

    pair = "[9,[8,7]]".parse().unwrap();
    assert_eq!(
        SnailFishPair::new(
            Item::RegularNumber(9),
            Item::Pair(SnailFishPair::new(
                Item::RegularNumber(8),
                Item::RegularNumber(7)
            ))
        ),
        pair
    );

    pair = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        .parse()
        .unwrap();

    let expected = SnailFishPair::new(
        Item::Pair(SnailFishPair::new(
            Item::Pair(SnailFishPair::new(
                Item::Pair(SnailFishPair::new(1.into(), 3.into())),
                Item::Pair(SnailFishPair::new(5.into(), 3.into())),
            )),
            Item::Pair(SnailFishPair::new(
                Item::Pair(SnailFishPair::new(1.into(), 3.into())),
                Item::Pair(SnailFishPair::new(8.into(), 7.into())),
            )),
        )),
        Item::Pair(SnailFishPair::new(
            Item::Pair(SnailFishPair::new(
                Item::Pair(SnailFishPair::new(4.into(), 9.into())),
                Item::Pair(SnailFishPair::new(6.into(), 9.into())),
            )),
            Item::Pair(SnailFishPair::new(
                Item::Pair(SnailFishPair::new(8.into(), 2.into())),
                Item::Pair(SnailFishPair::new(7.into(), 3.into())),
            )),
        )),
    );
    assert_eq!(expected, pair);
}

#[test]
fn test_explode() {
    // let snf: SnailFishPair = "[[[[[9,8],1],2],3],4]".parse().unwrap();
    // let snf: SnailFishPair = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
    // let snf: SnailFishPair = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
    let snf: SnailFishPair = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
    // let snf: SnailFishPair = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
    
    let mut flattened: Vec<(i8, Item)> = Vec::new();
    flatten(snf, &mut flattened, 0);

    println!("before -> {:?}", flattened);

    let exploded = explode(&mut flattened);

    println!("after -> {:?}", exploded);
}

#[test]
fn test_split() {
    let snf: SnailFishPair = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();

    let mut flattened: Vec<(i8, Item)> = Vec::new();
    flatten(snf, &mut flattened, 0);
    let exploded = explode(&mut flattened);

    println!("before split:\n{:?}", exploded);

    let after_split = split(&exploded);
    
    println!("after split\n{:?}", after_split);
}

#[test]
fn test_step() {
    let snf: SnailFishPair = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
    let mut flattened: Vec<(i8, Item)> = Vec::new();
    flatten(snf, &mut flattened, 0);
    
    let results = step(&flattened);

    println!("after step:\n{:?}", results);
}

fn flatten(snf: SnailFishPair, accum: &mut Vec<(i8, Item)>, depth: i8) {
    let l = *snf.left;
    let r = *snf.right;

    flatten_item(l, accum, depth);
    flatten_item(r, accum, depth);
}

fn flatten_item(item: Item, accum: &mut Vec<(i8, Item)>, depth: i8) {
    match item {
        Item::Pair(snf) => flatten(snf, accum, depth + 1),
        other => {
            accum.push((depth, other));
        },        
    }
}

fn to_num(item: &Item) -> i64 {
    match item {
        Item::RegularNumber(n) => *n,
        _ => 0
    }
}

fn step(items: &Vec<(i8, Item)>) -> Vec<(i8, Item)> {
    let mut initial = items.clone();
    loop {
        let after_explode = explode(&mut initial);
        let after_split = split(&after_explode);
        if after_explode.len() == initial.len() && after_split.len() == initial.len() {
            return after_split;
        }
        initial = after_split;
    }
}

fn explode(flattened: &mut Vec<(i8, Item)>) -> Vec<(i8, Item)> {
    let mut on_left = false;
    for (index, (depth, item)) in flattened.clone().iter().enumerate() {
        if *depth == 4 {
            on_left = !on_left;
            if on_left {
                println!("left 4: {:?}", item);
                if index > 0 {
                    let left_of = &flattened[index - 1].1;
                    let sum = to_num(item) + to_num(left_of);
                    println!("left of -> {:?} - sum -> {:?}", left_of, sum);
                    flattened[index - 1].1 = Item::RegularNumber(sum);
                }
                flattened[index] = (3, Item::RegularNumber(0));
            } else {
                println!("right 4: {:?}", item);
                if index + 1 < flattened.len() {
                    let right_of = &flattened[index + 1].1;
                    let sum = to_num(item) + to_num(right_of);
                    println!("right of -> {:?} - sum -> {:?}", right_of, sum);
                    flattened[index + 1].1 = Item::RegularNumber(sum);
                }
            }
        }
    }

    let mut filtered: Vec<(i8, Item)> = Vec::new();
    for (depth, item) in flattened {
        if *depth < 4 {
            filtered.push((*depth, item.clone()));
        }
    }
    filtered
}

fn split(to_split: &Vec<(i8, Item)>) -> Vec<(i8, Item)> {
    let mut to_return: Vec<(i8, Item)> = Vec::new();

    let mut has_split = false;

    for (depth, item) in to_split {
        let n = to_num(item);
        if !has_split && n > 9 {
            let half_down = n / 2;
            let half_up = (n / 2) + (n % 2);
            // println!("splitting {} to {} and {}", n, half_down, half_up);
            to_return.push((*depth + 1, Item::RegularNumber(half_down)));
            to_return.push((*depth + 1, Item::RegularNumber(half_up)));
            has_split = true;
        } else {
            to_return.push((*depth, item.clone()));
        }
    }

    to_return
}

#[test]
fn test_magnitude() {
    // let snf: SnailFishPair = "[[1,2],[[3,4],5]]".parse().unwrap();
    let snf: SnailFishPair = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();
    let mut flattened: Vec<(i8, Item)> = Vec::new();
    flatten(snf, &mut flattened, 0);

    println!("flattened\n{:?}", flattened);

    let result = magnitude(&flattened, 3);

    println!("result: {:?}", result);
}

fn magnitude(items: &Vec<(i8, Item)>, level: i8) -> Vec<(i8, Item)> {
    if level < 0 {
        items.clone()
    } else {
        let mut next = Vec::new();
        let mut pair: Vec<i64> = Vec::new();

        for (depth, item) in items {
            if *depth == level {
                if let Some(left) = pair.pop() {
                    let m = (3*left)+(2*to_num(item));
                    next.push((level - 1, Item::RegularNumber(m)));
                } else {
                    pair.push(to_num(item));
                }
            } else {
                next.push((*depth, item.clone()))
            }
        }

        magnitude(&next, level - 1)
    }
}

fn combine(left: &Vec<(i8, Item)>, right: &Vec<(i8, Item)>) -> Vec<(i8, Item)> {
    let mut combined = Vec::new();

    for (depth, item) in left {
        combined.push((depth + 1, item.clone()));
    }

    for (depth, item) in right {
        combined.push((depth + 1, item.clone()));
    }

    combined
}

#[test]
fn test_combine() {
    let left = "[[[[4,3],4],4],[7,[[8,4],9]]]";
    let right = "[1,1]";

    let sfp_l: SnailFishPair = left.parse().unwrap();
    let sfp_r: SnailFishPair = right.parse().unwrap();

    let mut flattened_l: Vec<(i8, Item)> = Vec::new();
    flatten(sfp_l, &mut flattened_l, 0);

    let mut flattened_r: Vec<(i8, Item)> = Vec::new();
    flatten(sfp_r, &mut flattened_r, 0);

    let combined = combine(&flattened_l, &flattened_r);
    println!("comb: {:?}", combined);

    let result = step(&combined);
    println!("r: {:?}", result);
}

#[test]
fn test_combine2() {
    // THIS IS BROKEN
    // gives      [[[[0,0],[5,3]],[4,4]],[5,5]]
    // instead of [[[[3,0],[5,3]],[4,4]],[5,5]]

    let left = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
    let right = "[5,5]";

    let sfp_l: SnailFishPair = left.parse().unwrap();
    let sfp_r: SnailFishPair = right.parse().unwrap();

    let mut flattened_l: Vec<(i8, Item)> = Vec::new();
    flatten(sfp_l, &mut flattened_l, 0);
    println!("left\n{:?}", flattened_l);

    let mut flattened_r: Vec<(i8, Item)> = Vec::new();
    flatten(sfp_r, &mut flattened_r, 0);
    println!("right\n{:?}", flattened_r);

    let combined = combine(&flattened_l, &flattened_r);
    println!("comb:\n{:?}", combined);

    let result = step(&combined);
    println!("r:\n{:?}", result);
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SnailFishPair {
    left: Box<Item>,
    right: Box<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Item {
    RegularNumber(i64),
    Pair(SnailFishPair),
}

impl SnailFishPair {
    fn new(left: Item, right: Item) -> SnailFishPair {
        SnailFishPair {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

impl FromStr for SnailFishPair {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;

        let mut items: Vec<Item> = Vec::new();

        for c in s.chars() {
            match c {
                '[' => (),
                ']' => {
                    let right = items.pop().unwrap();
                    let left = items.pop().unwrap();
                    items.push(Item::Pair(SnailFishPair::new(left, right)));
                }
                ',' => (),
                n => {
                    let i: i64 = n.to_digit(RADIX).unwrap() as i64;
                    items.push(Item::RegularNumber(i));
                }
            }
        }
        let pair = items.pop().unwrap();
        match pair {
            Item::Pair(snp) => Ok(snp),
            _ => Err(ParseErr),
        }
    }
}

impl From<i64> for Item {
    fn from(i: i64) -> Self {
        Item::RegularNumber(i)
    }
}