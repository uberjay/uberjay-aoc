use failure::{err_msg, Error};
use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split_whitespace().map(|s| s.parse()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> Result<i64, Error> {
    let mut set: std::collections::HashSet<i64> = std::collections::HashSet::default();
    let mut freq = 0;

    input
        .iter()
        .cycle()
        .find_map(|x| {
            if set.insert(freq) {
                freq += x;
                None
            } else {
                Some(freq)
            }
        })
        .ok_or(err_msg("failed to find repeating frequency"))
}

#[aoc(day1, part2, fxhash)]
pub fn solve_part2_fxhash(input: &[i64]) -> Result<i64, Error> {
    let mut set = fxhash::FxHashSet::default();
    let mut freq = 0;

    input
        .iter()
        .cycle()
        .find_map(|x| {
            if set.insert(freq) {
                freq += x;
                None
            } else {
                Some(freq)
            }
        })
        .ok_or(err_msg("failed to find repeating frequency"))
}

#[test]
pub fn test_part1() {
    assert_eq!(solve_part2(&[1, -1]).unwrap(), 0);
}
