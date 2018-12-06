use failure::Error;
use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn mh_dist_to(&self, x: i32, y: i32) -> u32 {
        ((self.x - x).abs() + (self.y - y).abs()) as u32
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pt_iter = s.split(", ").map(|s| s.parse().unwrap());
        Ok(Self {
            x: pt_iter.next().unwrap(),
            y: pt_iter.next().unwrap(),
        })
    }
}

#[test]
fn test_mh_dist() {
    let pt = Point { x: 10, y: 10 };
    assert_eq!(pt.mh_dist_to(10, 10), 0);
    assert_eq!(pt.mh_dist_to(9, 10), 1);
    assert_eq!(pt.mh_dist_to(11, 10), 1);

    assert_eq!(pt.mh_dist_to(10, 10), 0);
    assert_eq!(pt.mh_dist_to(10, 9), 1);
    assert_eq!(pt.mh_dist_to(10, 11), 1);

    assert_eq!(pt.mh_dist_to(9, 11), 2);
    assert_eq!(pt.mh_dist_to(11, 9), 2);
}

#[test]
fn test_sample() {
    let points = input_generator("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9").unwrap();
    assert_eq!(solve_part1(&points).unwrap(), 17);
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Vec<Point>, Error> {
    input.lines().map(|l| l.parse()).collect()
}

fn mh_dist_to_points(
    grid_iter: impl Iterator<Item = (i32, i32)>,
    points: &[Point],
) -> Vec<(Point, Point)> {
    let grid_points: Vec<(i32, i32)> = grid_iter.collect();

    grid_points
        .par_iter()
        .filter_map(|&(x, y)| {
            // for each point in the bounding box, find the distance to each input
            // point.
            let mut dists: Vec<(Point, u32)> = points
                .iter()
                .enumerate()
                .map(|(_idx, pt)| (*pt, pt.mh_dist_to(x, y)))
                .collect();

            dists.par_sort_unstable_by_key(|(_k, dist)| *dist);

            // only collect points which have a unique nearest-input-point
            match (dists.get(0), dists.get(1)) {
                (Some((pa, _dist)), None) => Some((*pa, Point { x, y })),
                (Some((pa, da)), Some((_pb, db))) => {
                    if da < db {
                        Some((*pa, Point { x, y }))
                    } else {
                        // current coordinate has a tie for nearest-input-point, so
                        // it doesn't count.
                        None
                    }
                }
                _ => panic!("failed to process grid location ({}, {})", x, y),
            }
        })
        .collect()
}

fn xy_points_minmax(points: &[Point]) -> (Point, Point) {
    let x_bounds = points.iter().minmax_by_key(|p| p.x).into_option().unwrap();
    let y_bounds = points.iter().minmax_by_key(|p| p.y).into_option().unwrap();

    let x_minmax = ((x_bounds.0.x), (x_bounds.1.x));
    let y_minmax = ((y_bounds.0.y), (y_bounds.1.y));

    (
        Point {
            x: x_minmax.0,
            y: y_minmax.0,
        },
        Point {
            x: x_minmax.1,
            y: y_minmax.1,
        },
    )
}

#[aoc(day6, part1)]
pub fn solve_part1(points: &[Point]) -> Result<usize, Error> {
    let (top_left, bot_right) = xy_points_minmax(points);
    let x_range = (top_left.x)..=(bot_right.x);
    let y_range = (top_left.y)..=(bot_right.y);

    let distances = mh_dist_to_points(x_range.cartesian_product(y_range), points);
    let nearest_by_point = distances.into_iter().into_group_map();

    Ok(nearest_by_point
        .values()
        .filter_map(|nearby| {
            if nearby.iter().all(|pt| {
                pt.x != top_left.x
                    && pt.x != bot_right.x
                    && pt.y != top_left.y
                    && pt.y != bot_right.y
            }) {
                Some(nearby.len())
            } else {
                None
            }
        })
        .max()
        .unwrap())
}

fn sum_mh_dist_to_point(x: i32, y: i32, points: &[Point]) -> u32 {
    points.iter().map(|pt| pt.mh_dist_to(x, y)).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(points: &[Point]) -> usize {
    let (tl, br) = xy_points_minmax(points);

    // evaluate each point on the grid -- if the sum of the manhattan distances
    // to all of the input points is less than 10000, that point is "safe". the
    // solution for part2 is the area which is considered safe.
    ((tl.x)..=(br.x))
        .cartesian_product((tl.y)..=(br.y))
        .filter_map(|(x, y)| match sum_mh_dist_to_point(x, y, points) {
            0..=9999 => Some(Point { x, y }),
            _ => None,
        })
        .count()
}
