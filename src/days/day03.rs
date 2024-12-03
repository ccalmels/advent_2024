use std::io::{BufRead, Lines};
use std::str::FromStr;
use regex::Regex;

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re2 = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut multiply = true;

    for line in lines {
	let line = line.unwrap();

	part1 = re.captures_iter(&line).fold(part1, |acc, c| {
	    let x = u32::from_str(c.get(1).map_or("", |m| m.as_str())).unwrap();
	    let y = u32::from_str(c.get(2).map_or("", |m| m.as_str())).unwrap();

	    acc + x * y
	});

	part2 = re2.captures_iter(&line).fold(part2, |acc, c| {
	    match c.get(0).unwrap().as_str() {
		"do()" => multiply = true,
		"don't()" => multiply = false,
		_ => {
		    if multiply {
			let x = u32::from_str(c.get(1).map_or("", |m| m.as_str())).unwrap();
			let y = u32::from_str(c.get(2).map_or("", |m| m.as_str())).unwrap();

			return acc + x * y;
		    }
		}
	    }
	    acc
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
