#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split_whitespace().map(|r| r.to_owned()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> i64 {
    let mut num_2 = 0;
    let mut num_3 = 0;

    for box_id in input {
        let mut lfq = fxhash::FxHashMap::default();

        box_id.chars().for_each(|c| {
            *(lfq.entry(c).or_insert(0)) += 1;
        });

        if lfq.values().any(|c| *c == 2) {
            num_2 += 1;
        }
        if lfq.values().any(|c| *c == 3) {
            num_3 += 1;
        }
    }

    num_2 * num_3
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
    for id1 in input {
        for id2 in input {
            if let Ok(1) = strsim::hamming(id1, id2) {
                return id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|(cha, chb)| cha == chb)
                    .map(|(cha, _)| cha)
                    .collect::<String>();
            }
        }
    }

    "nopenopenopenope".to_owned()
}
