use arrayvec::ArrayVec;
use failure::{bail, err_msg, Error};
use hashbrown::HashMap;
use petgraph::prelude::*;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Params {
    players: u32,
    last_marble: u64,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Box<Params>, Error> {
    let mut it = input.split(' ');
    Ok(Box::new(Params {
        players: it
            .next()
            .ok_or(err_msg("missing number of players"))?
            .parse()?,
        last_marble: it
            .skip(5)
            .next()
            .ok_or(err_msg("missing last marble worth"))?
            .parse()?,
    }))
}

#[derive(Clone, Debug)]
struct Board<T> {
    storage: VecDeque<T>,
}

impl<T> Board<T> {
    pub fn new() -> Board<T> {
        Board {
            storage: VecDeque::new(),
        }
    }

    pub fn add_marble(&mut self, value: T) {
        self.storage.push_front(value);
    }

    pub fn remove_marble(&mut self) -> Option<T> {
        self.storage.pop_front()
    }

    pub fn rotate(&mut self, offset: isize) {
        if offset == 0 || self.storage.len() <= 1 {
            return;
        }

        if offset < 0 {
            for _ in 0..(-offset) {
                self.prev();
            }
        } else {
            for _ in 0..offset {
                self.next();
            }
        }
    }

    fn next(&mut self) {
        if let Some(v) = self.storage.pop_front() {
            self.storage.push_back(v);
        }
    }

    fn prev(&mut self) {
        if let Some(v) = self.storage.pop_back() {
            self.storage.push_front(v);
        }
    }
}

#[derive(Debug)]
struct Game {
    board: Board<u64>,
    next_marble: u64,
    cur_player: u32,
    params: Params,
    scores: HashMap<u32, u64>,
}

impl Game {
    pub fn new(params: Params) -> Game {
        let mut board = Board::new();
        board.add_marble(0);

        Game {
            board,
            params,
            next_marble: 1,
            cur_player: 0,
            scores: HashMap::new(),
        }
    }

    pub fn complete(&self) -> bool {
        self.next_marble > self.params.last_marble
    }

    pub fn play_turn(&mut self) -> Option<u64> {
        if self.complete() {
            return None;
        }

        if (self.next_marble % 23) == 0 {
            self.board.rotate(-7);

            *self.scores.entry(self.cur_player).or_insert(0) +=
                self.next_marble + self.board.remove_marble().unwrap();
        } else {
            self.board.rotate(2);
            self.board.add_marble(self.next_marble);
        }

        self.next_marble += 1;
        self.cur_player = (self.cur_player + 1) % self.params.players;

        Some(self.next_marble - 1)
    }

    pub fn winning_score(&self) -> Result<u64, Error> {
        Ok(*self
            .scores
            .values()
            .max()
            .ok_or(err_msg("no player scored any points"))?)
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(params: &Params) -> Result<u64, Error> {
    let mut game = Game::new(*params);

    while !game.complete() {
        game.play_turn().unwrap();
    }

    game.winning_score()
}

#[test]
fn test_part1_sample() {
    let samples: [(&str, u64); 6] = [
        (&"9 players; last marble is worth 25 points", 32),
        (&"10 players; last marble is worth 1618 points", 8317),
        (&"13 players; last marble is worth 7999 points", 146373),
        (&"17 players; last marble is worth 1104 points", 2764),
        (&"21 players; last marble is worth 6111 points", 54718),
        (&"30 players; last marble is worth 5807 points", 37305),
    ];

    for sample in &samples {
        let params = input_generator(sample.0).unwrap();
        assert_eq!(solve_part1(&params).unwrap(), sample.1);
    }
}

#[aoc(day9, part2)]
pub fn solve_part2(params: &Params) -> Result<u64, Error> {
    let big_params = Params {
        players: params.players,
        last_marble: params.last_marble * 100,
    };

    let mut game = Game::new(big_params);

    while !game.complete() {
        game.play_turn().unwrap();
    }

    game.winning_score()
}
