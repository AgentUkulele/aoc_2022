#![allow(unused_mut, unused_variables)]

use std::collections::VecDeque;

#[cfg(not(test))]
static FILE: &'static str = include_str!("./day5.prod");

#[cfg(test)]
static FILE: &'static str = include_str!("./day5.test");

pub fn day5_1() -> String {

    println!("this is chunk #1");
    // let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let (mut graph, instructions) = FILE.split_once("\n\n").unwrap();

    let mut lines = graph.lines().rev();

    let length = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .len();

    println!("{length}");

    // let mut stacks: [VecDeque<String>; length] = Vec::new();

    println!("{graph}");
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5_1() {
        assert_eq!(day5_1(), "CMZ");
    }
}
