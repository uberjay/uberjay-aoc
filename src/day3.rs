use failure::Error;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Claim {
    pub id: u16,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '#').split(" @ ").collect();
        let parts2: Vec<&str> = parts[1].split(": ").collect();
        let xy: Vec<&str> = parts2[0].split(',').collect();
        let wh: Vec<&str> = parts2[1].split('x').collect();

        Ok(Self {
            id: parts[0].parse()?,
            x: xy[0].parse()?,
            y: xy[1].parse()?,
            w: wh[0].parse()?,
            h: wh[1].parse()?,
        })
    }
}

impl Claim {
    pub fn iter_points(&self) -> impl Iterator<Item = (u16, u16)> {
        (self.x..(self.x + self.w)).cartesian_product(self.y..(self.y + self.h))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|r| r.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> usize {
    let mut claim_map: [[u16; 1000]; 1000] = [[0; 1000]; 1000];
    let mut tot = 0;

    for claim in input {
        for (x, y) in claim.iter_points() {
            let entry = &mut claim_map[x as usize][y as usize];
            *entry += 1;

            if *entry == 2 {
                tot += 1;
            }
        }
    }

    tot
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> u16 {
    let mut claim_map: [[u16; 1000]; 1000] = [[0; 1000]; 1000];
    let mut candidates = hashbrown::HashSet::new();

    for claim in input {
        candidates.insert(claim.id);
    }

    for claim in input {
        for (x, y) in claim.iter_points() {
            let entry = &mut claim_map[x as usize][y as usize];

            if *entry != 0 {
                candidates.remove(&claim.id);
                candidates.remove(entry);
            } else {
                *entry = claim.id;
            }
        }
    }

    assert_eq!(candidates.len(), 1);

    candidates.into_iter().next().unwrap()
}
