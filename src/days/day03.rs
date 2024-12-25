use regex::Regex;
use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (u32, u32, bool)
where
    T: BufRead,
{
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    lines.fold((0, 0, true), |(part1, part2, multiply), line| {
        re.captures_iter(&line.unwrap())
            .fold((part1, part2, multiply), |(p1, p2, m), c| {
                match c.get(0).unwrap().as_str() {
                    "do()" => (p1, p2, true),
                    "don't()" => (p1, p2, false),
                    _ => {
                        let v = c.get(1).map_or("", |m| m.as_str()).parse::<u32>().unwrap()
                            * c.get(2).map_or("", |m| m.as_str()).parse::<u32>().unwrap();

                        (p1 + v, if m { p2 + v } else { p2 }, m)
                    }
                }
            })
    })
}

#[test]
fn check() {
    const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (161, 48, true));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
