use failure::Error;
use itertools::Itertools;
use petgraph::prelude::*;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(char, char, u32)> {
    input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            let a = parts[1].chars().next().unwrap();
            let b = parts[7].chars().next().unwrap();
            let weight = (b as i8 - b'Z' as i8).abs() as u32;
            (a, b, weight)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(edges: &[(char, char, u32)]) -> Result<String, Error> {
    let mut graph = DiGraphMap::<_, u32>::from_edges(edges);
    let mut seq: Vec<char> = Vec::new();

    loop {
        if graph.node_count() == 0 {
            break;
        }

        let options: Vec<char> = graph
            .nodes()
            .filter(|n| graph.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .sorted();

        let step = options[0];
        seq.push(step);

        if !graph.remove_node(step) {
            panic!("graph doesn't contain node {}", step);
        }
    }

    Ok(seq.iter().collect())
}

#[test]
fn test_part1_sample() {
    let edges = input_generator("Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n");

    assert_eq!(solve_part1(&edges).unwrap(), "CABDFE".to_owned());
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Worker {
    Idle,
    Busy(char, u8),
}

#[aoc(day7, part2)]
pub fn solve_part2(edges: &[(char, char, u32)]) -> Result<usize, Error> {
    let mut graph = DiGraphMap::<_, u32>::from_edges(edges);
    let mut workers: [Worker; 5] = [Worker::Idle; 5];
    let mut ticks = 0;

    loop {
        if graph.node_count() == 0 {
            if workers.iter().all(|&v| v == Worker::Idle) {
                break;
            }
        }

        let options: Vec<char> = graph
            .nodes()
            .filter(|n| graph.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .sorted();

        'next: for option in options {
            for worker in &workers {
                if let Worker::Busy(o, _) = worker {
                    if *o == option {
                        continue 'next;
                    }
                }
            }
            match workers.iter_mut().find(|w| **w == Worker::Idle) {
                Some(worker) => {
                    let dur = (option as u8) - b'A' + 61;
                    *worker = Worker::Busy(option, dur);
                }
                None => (),
            }
        }

        ticks += 1;
        workers.iter_mut().for_each(|w| {
            if let Worker::Busy(c, d) = *w {
                if d == 1 {
                    *w = Worker::Idle;
                    graph.remove_node(c);
                } else {
                    *w = Worker::Busy(c, d - 1);
                }
            }
        });
    }

    Ok(ticks)
}
