use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::iter::zip;
use std::str::FromStr;

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let (mut left, mut right) = (vec![], vec![]);
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let (a, b) = (
            u32::from_str(split[0]).unwrap(),
            u32::from_str(split[1]).unwrap(),
        );

        left.push(a);
        right.push(b);

        *counts.entry(b).or_insert(0) += 1;
    }

    let numbers = left.clone();

    left.sort_unstable();
    right.sort_unstable();

    (
        zip(left, right).fold(0, |acc, (a, b)| acc + a.abs_diff(b)),
        numbers.iter().fold(0, |acc, e| {
            if let Some(count) = counts.get(e) {
                acc + e * count
            } else {
                acc
            }
        }),
    )
}

#[test]
fn check() {
    const TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (11, 31));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
