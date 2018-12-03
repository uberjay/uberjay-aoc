use failure::Error;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '#').split(" @ ").collect();
        let parts2: Vec<&str> = parts[1].split(": ").collect();
        let xy: Vec<&str> = parts2[0].split(',').collect();
        let wh: Vec<&str> = parts2[1].split('x').collect();

        Ok(Self {
            id: parts[0].parse::<u32>()?,
            x: xy[0].parse::<u32>()?,
            y: xy[1].parse::<u32>()?,
            w: wh[0].parse::<u32>()?,
            h: wh[1].parse::<u32>()?,
        })
    }
}

impl Claim {
    pub fn covers(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in self.x..(self.x + self.w) {
            for y in self.y..(self.y + self.h) {
                points.push(Point { x, y });
            }
        }
        points
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|r| r.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> usize {
    let mut claim_map: HashMap<Point, u32> = HashMap::default();

    for claim in input {
        for pt in claim.covers() {
            (*claim_map.entry(pt).or_insert(0)) += 1;
        }
    }

    claim_map.values().filter(|v| **v > 1).count()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> u32 {
    let mut claim_map: HashMap<Point, Vec<&Claim>> = HashMap::default();
    let mut candidates: HashSet<u32> = HashSet::default();

    for claim in input {
        candidates.insert(claim.id);

        for pt in claim.covers() {
            let cvec = claim_map.entry(pt).or_insert(Vec::new());
            cvec.push(&claim);
        }
    }

    claim_map.values().filter(|v| v.len() > 1).for_each(|cl| {
        cl.iter().for_each(|c| {
            candidates.remove(&c.id);
        });
    });

    assert_eq!(candidates.len(), 1);

    candidates.into_iter().next().unwrap()
}
