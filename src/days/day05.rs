use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

#[derive(Debug, Eq, PartialEq)]
struct Page {
    number: u32,
    before: HashSet<u32>,
}

impl Page {
    fn new(number: u32) -> Self {
        let before = HashSet::new();

        Page { number, before }
    }

    fn add_print_before(&mut self, number: u32) {
        self.before.insert(number);
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.before.contains(&other.number) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[test]
fn check_page() {
    let mut p47 = Page::new(47);
    let p53 = Page::new(53);

    assert!(!(p47 < p53));

    p47.add_print_before(p53.number);

    assert!(p47 < p53);

    let mut p75 = Page::new(75);

    p75.add_print_before(47);
    p75.add_print_before(61);

    let p61 = Page::new(61);

    assert!(p75 < p47);
    assert!(p75 < p61);
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut lines = lines;
    let mut pages = HashMap::new();

    loop {
        let line = lines.next().unwrap().unwrap();

        if line.is_empty() {
            break;
        }

        let splitted: Vec<u32> = line.split('|').map(|s| s.parse().unwrap()).collect();

        pages
            .entry(splitted[0])
            .or_insert(Page::new(splitted[0]))
            .add_print_before(splitted[1]);

        pages.entry(splitted[1]).or_insert(Page::new(splitted[1]));
    }

    lines.fold((0, 0), |(p1, p2), line| {
        let pages_list: Vec<&Page> = line
            .unwrap()
            .split(',')
            .map(|s| pages.get(&s.parse().unwrap()).unwrap())
            .collect();

        if pages_list.is_sorted() {
            (p1 + pages_list[pages_list.len() / 2].number, p2)
        } else {
            let mut pages_list = pages_list;

            pages_list.sort_unstable();

            (p1, p2 + pages_list[pages_list.len() / 2].number)
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
