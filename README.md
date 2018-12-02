# Advent of Code, 2018

This repo contains my [Advent of Code 2018](https://adventofcode.com/2018) solutions. This is the first year I'm participating in AoC, and I'm writing my solutions in [Rust](https://rust-lang.org). I'm not going for the most idiomatic style. (I don't have enough experience to claim that I'm writing idiomatically!)

This is mostly an exercise to gain more familiarity with the standard library and whatever external crates I come across. Additionally, I'm using the current Rust beta with the [2018 edition](https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html) enabled. Also, it's fun! :)

I'm using the [cargo-aoc](https://github.com/gobanos/cargo-aoc) helper tool and associated crates. It's quite nice for making quick and dirty solutions, but isn't oriented towards creating a command-line tool with proper error handling and such. If you're looking for those sorts of solutions, perhaps you should take a look at [BurntSushi's repository](https://github.com/BurntSushi/advent-of-code).

## Solution structure

Each solution's code is in src/day<N>.rs. `cargo-aoc` allows one to define so-called "generator" functions which provide for a shared way of pre-processing a particular day's input file. If you look at the solutions for day1, for example, the generator is responsible for parsing the input file into a Vec<i64>:

```rust
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
```

This allows each solution to just consume the parsed vector. For example, day1, part1 looks like this:

```rust
#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    input.iter().sum()
}
```

There may be multiple solutions for each day/part for experimenting with different approaches. For example, I originally used a HashSet for day1/part2, and was later curious what the effective improvement from using FxHashSet would be:

```rust
#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
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
        .unwrap()
}

#[aoc(day1, part2, fxhash)]
pub fn solve_part2_fxhash(input: &[i64]) -> i64 {
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
        .unwrap()
}
```

## Benchmarking

Using the `cargo aoc bench` utility makes benchmarking (with the fantastic [criterion](https://github.com/japaric/criterion.rs)) easy, so *why not*? With this, we can see that using FxHashSet instead of HashSet finds the solution quite a bit faster. Instead of 16.4ms, it's 7.8ms:

```shell
❯ cargo aoc bench -d1 -p2
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
   Compiling aoc-autobench v0.1.0 (/Users/huber/code/github.com/uberjay/uberjay-aoc/target/aoc/aoc-autobench)
    Finished release [optimized] target(s) in 1.91s
     Running target/release/deps/aoc_benchmark-0057d72856d8c64a
Day1 - Part2/(default)  time:   [16.409 ms 16.485 ms 16.567 ms]
                        change: [-2.3522% -1.2570% -0.1511%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Day1 - Part2/fxhash     time:   [7.8588 ms 7.8970 ms 7.9351 ms]
                        change: [-3.2052% -1.4764% +0.2960%] (p = 0.10 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
```

Criterion generates really nice html reports for easily comparing results between solutions, as well as historical changes for any given solution. ([example comparing my two solutions for day1/part2](https://www.paradoxical.net/~huber/criterion-example/Day1%20-%20Part2/report/))
