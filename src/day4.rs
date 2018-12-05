use chrono::prelude::*;
use failure::{bail, Error};
use hashbrown::HashMap;
use itertools::Itertools;
use std::ops::Range;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ShiftRecord {
    pub guard: u32,
    events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Start(DateTime<Utc>),
    Wake(DateTime<Utc>),
    Sleep(DateTime<Utc>),
}

impl FromStr for ShiftRecord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dt, desc) = ShiftRecord::extract_time_and_desc(s)?;
        let desc_parts: Vec<&str> = desc.split('#').collect();

        Ok(Self {
            guard: desc_parts[1].split_whitespace().next().unwrap().parse()?,
            events: vec![Event::Start(dt)],
        })
    }
}

impl ShiftRecord {
    pub fn from_lines(lines: &[&str]) -> Result<Self, Error> {
        let mut shift: ShiftRecord = lines[0].parse().unwrap();
        shift.add_events(&lines[1..]).unwrap();
        Ok(shift)
    }

    fn extract_time_and_desc<'a>(line: &'a str) -> Result<(DateTime<Utc>, &'a str), Error> {
        let parts: Vec<&str> = line.split("] ").collect();
        let time_str = parts[0].trim_left_matches('[');
        Ok((Utc.datetime_from_str(time_str, "%Y-%m-%d %H:%M")?, parts[1]))
    }

    pub fn add_events(&mut self, lines: &[&str]) -> Result<(), Error> {
        for line in lines {
            let (dt, desc) = ShiftRecord::extract_time_and_desc(line)?;
            match desc {
                "wakes up" => self.events.push(Event::Wake(dt)),
                "falls asleep" => self.events.push(Event::Sleep(dt)),
                _ => bail!("unexpected event description '{}'", desc),
            }
        }
        Ok(())
    }

    pub fn minutes_slept(&self) -> u64 {
        let start_ev = &self.events[0];

        self.events
            .iter()
            .skip(1)
            .fold((0, start_ev), |(mins, prev_ev), ev| match (prev_ev, ev) {
                (&Event::Sleep(st), &Event::Wake(wt)) => {
                    (mins + (wt - st).num_minutes() as u64, ev)
                }
                _ => (mins, ev),
            })
            .0
    }

    pub fn sleep_ranges(&self) -> Vec<Range<u32>> {
        let start_ev = &self.events[0];
        let mut ranges = Vec::new();

        self.events
            .iter()
            .skip(1)
            .fold(start_ev, |prev_ev, ev| match (prev_ev, ev) {
                (&Event::Sleep(st), &Event::Wake(wt)) => {
                    ranges.push(st.minute()..wt.minute());
                    ev
                }
                _ => ev,
            });

        ranges
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<ShiftRecord> {
    let lines: Vec<&str> = input.lines().sorted();
    let sorted_lines_it = lines.iter().cloned().peekable();
    let raw_shifts = sorted_lines_it.batching(|it| match it.next() {
        None => None,
        Some(x) if x.contains(" begins shift") => {
            let mut t = vec![x];
            t.extend(it.peeking_take_while(|l| !l.contains(" begins shift")));
            Some(t)
        }
        Some(x) => panic!("unexpected line in input: '{}'", x),
    });

    raw_shifts
        .map(|shift_lines| ShiftRecord::from_lines(&shift_lines).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[ShiftRecord]) -> u32 {
    let all_shifts: Vec<&ShiftRecord> = input.iter().collect();
    let mut total_slept = HashMap::<u32, u64>::new();

    // accumulate sleep time for all shifts keyed by guard id
    for shift in all_shifts {
        *total_slept.entry(shift.guard).or_insert(0) += shift.minutes_slept();
    }

    // find the sleepiest guard.
    let (guard, _mins) = total_slept.into_iter().max_by_key(|&(_gid, m)| m).unwrap();

    // now figure out which is the most likely minute for them to be sleeping.
    let shifts: Vec<&ShiftRecord> = input.iter().filter(|sr| sr.guard == guard).collect();
    let mut sleep_map: Vec<u32> = vec![0; 60];

    for shift in shifts {
        for rg in shift.sleep_ranges() {
            for m in rg {
                sleep_map[m as usize] += 1;
            }
        }
    }

    let sleepiest_minute = sleep_map
        .iter()
        .enumerate()
        .max_by_key(|&(_idx, v)| v)
        .unwrap()
        .0 as u32;

    sleepiest_minute * guard
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[ShiftRecord]) -> u32 {
    let all_shifts: Vec<&ShiftRecord> = input.iter().collect();
    let mut sleep_map = HashMap::<u32, HashMap<u32, u32>>::new();

    for shift in all_shifts {
        for rg in shift.sleep_ranges() {
            for m in rg {
                let min_entry = sleep_map.entry(m).or_insert(HashMap::new());
                let count_entry = min_entry.entry(shift.guard).or_insert(0);
                *count_entry += 1;
            }
        }
    }

    let minute = sleep_map
        .iter()
        .max_by_key(|&(_k, v)| v.values().max())
        .unwrap()
        .0;
    let min_entry = &sleep_map[minute];
    let guard = min_entry.iter().max_by_key(|&(_k, v)| v).unwrap().0;

    minute * guard
}
