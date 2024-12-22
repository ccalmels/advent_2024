use regex::Regex;
use std::io::{BufRead, Lines};
use std::str::FromStr;

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut multiply = true;

    for line in lines {
        let line = line.unwrap();

        (part1, part2) = re.captures_iter(&line).fold((part1, part2), |(p1, p2), c| {
            match c.get(0).unwrap().as_str() {
                "do()" => multiply = true,
                "don't()" => multiply = false,
                _ => {
                    let x = u32::from_str(c.get(1).map_or("", |m| m.as_str())).unwrap();
                    let y = u32::from_str(c.get(2).map_or("", |m| m.as_str())).unwrap();

                    if multiply {
                        return (p1 + x * y, p2 + x * y);
                    } else {
                        return (p1 + x * y, p2);
                    }
                }
            }
            (p1, p2)
        });
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (161, 48));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
