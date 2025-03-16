use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn split_stone(n: usize) -> Option<usize> {
    let mut count = 0;
    let mut temp = n;
    let mut mul = 1;

    while temp >= 1 {
        temp /= 10;
        count += 1;
        if count % 2 == 0 {
            mul *= 10;
        }
    }

    if count % 2 == 0 {
        Some(mul)
    } else {
        None
    }
}

#[test]
fn check_split_stone() {
    assert_eq!(split_stone(125), None);
    assert_eq!(split_stone(253000), Some(1000));
    assert_eq!(split_stone(2024), Some(100));
    assert_eq!(split_stone(28676032), Some(10000));

    assert_eq!(split_stone(1000), Some(100));
}

#[cfg(test)]
// tried this implementation, but it isn't faster
fn split_stone2(n: usize) -> Option<usize> {
    const CACHE: &[usize] = &[
        0,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
    ];

    let index = match CACHE.binary_search(&n) {
        Err(index) => index,
        Ok(index) => index + 1,
    };

    assert_ne!(index, CACHE.len());

    if index % 2 == 0 {
        Some(CACHE[index / 2])
    } else {
        None
    }
}

#[test]
fn check_split_stone2() {
    assert_eq!(split_stone2(125), None);
    assert_eq!(split_stone2(253000), Some(1000));
    assert_eq!(split_stone2(2024), Some(100));
    assert_eq!(split_stone2(28676032), Some(10000));

    assert_eq!(split_stone2(1000), Some(100));
}

fn number_of_stones(
    cache: &mut HashMap<(usize, usize), usize>,
    level: usize,
    stone: usize,
) -> usize {
    if level == 0 {
        1
    } else if let Some(&v) = cache.get(&(level, stone)) {
        v
    } else {
        let ret = if stone == 0 {
            number_of_stones(cache, level - 1, 1)
        } else if let Some(mul) = split_stone(stone) {
            number_of_stones(cache, level - 1, stone / mul)
                + number_of_stones(cache, level - 1, stone % mul)
        } else {
            number_of_stones(cache, level - 1, stone * 2024)
        };

        cache.insert((level, stone), ret);

        ret
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let line = lines.last().unwrap().unwrap();
    let stones: Vec<usize> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut cache = HashMap::new();

    (
        stones
            .iter()
            .fold(0, |acc, &s| acc + number_of_stones(&mut cache, 25, s)),
        stones
            .iter()
            .fold(0, |acc, &s| acc + number_of_stones(&mut cache, 75, s)),
    )
}

#[test]
fn check() {
    const TEST: &str = "125 17";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (55312, 65601038650482));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
