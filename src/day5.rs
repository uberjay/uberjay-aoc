use rayon::prelude::*;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_string()
}

fn reject_adjecent_pairs(chars: impl Iterator<Item = char>) -> String {
    chars
        .fold(Vec::new(), |mut nv: Vec<char>, b| {
            if let Some(a) = nv.last() {
                if *a != b && a.eq_ignore_ascii_case(&b) {
                    nv.pop();
                } else {
                    nv.push(b);
                }
            } else {
                nv.push(b);
            }
            nv
        })
        .iter()
        .collect::<String>()
}

#[test]
fn test_1() {
    assert_eq!(
        reject_adjecent_pairs("dabAcCaCBAcCcaDA".chars()),
        "dabCBAcaDA".to_owned()
    );
    assert_eq!(reject_adjecent_pairs("abBA".chars()), "".to_owned());
    assert_eq!(reject_adjecent_pairs("abAB".chars()), "abAB".to_owned());
    assert_eq!(reject_adjecent_pairs("aabAAB".chars()), "aabAAB".to_owned());
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    reject_adjecent_pairs(input.chars()).len()
}

const A_Z: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    A_Z.par_iter()
        .map(|p| reject_adjecent_pairs(input.chars().filter(|c| !c.eq_ignore_ascii_case(&p))).len())
        .min()
        .unwrap()
}
