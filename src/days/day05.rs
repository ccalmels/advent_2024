use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};
use std::str::FromStr;

fn less_than(ordering_rules: &HashMap<u32, HashSet<u32>>, a: &u32, b: &u32) -> Ordering {
    if let Some(hset) = ordering_rules.get(a) {
        if hset.contains(b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut lines = lines;
    let mut ordering_rules = HashMap::new();

    loop {
        let line = lines.next().unwrap().unwrap();

        if line.is_empty() {
            break;
        }

        let splitted: Vec<u32> = line.split('|').map(|s| u32::from_str(s).unwrap()).collect();

        ordering_rules
            .entry(splitted[0])
            .or_insert(HashSet::<u32>::new())
            .insert(splitted[1]);
    }

    lines.fold((0, 0), |(p1, p2), line| {
        let line = line.unwrap();
        let pages: Vec<u32> = line.split(',').map(|s| u32::from_str(s).unwrap()).collect();

        if pages.is_sorted_by(|a, b| less_than(&ordering_rules, a, b).is_le()) {
            (p1 + pages[pages.len() / 2], p2)
        } else {
            let mut pages = pages;

            pages.sort_unstable_by(|a, b| less_than(&ordering_rules, a, b));

            (p1, p2 + pages[pages.len() / 2])
        }
    })
}

#[test]
fn check() {
    const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (143, 123));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
