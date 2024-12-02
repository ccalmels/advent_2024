use std::io::{BufRead, Lines};
use std::str::FromStr;

fn is_safe(report: &[i32]) -> bool {
    let direction = (report[1] - report[0]).signum();

    report.iter().zip(report.iter().skip(1)).all(|(a, b)| {
        let d = b - a;

        if d.signum() != direction {
            return false;
        }

        if d.abs() < 1 || d.abs() > 3 {
            return false;
        }

        true
    })
}

#[test]
fn check_is_safe() {
    assert!(is_safe(&vec![7, 6, 4, 2, 1]));
    assert!(!is_safe(&vec![1, 2, 7, 8, 9]));
    assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
    assert!(!is_safe(&vec![8, 6, 4, 4, 1]));
}

fn resolve<T>(lines: Lines<T>) -> (u32, u32)
where
    T: BufRead,
{
    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let line = line.unwrap();
        let numbers = line
            .split(' ')
            .map(|n| i32::from_str(n).unwrap())
            .collect::<Vec<_>>();

        if is_safe(&numbers) {
            part1 += 1;
            part2 += 1;
        } else if (0..numbers.len()).any(|n| {
            is_safe(
                &numbers
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &e)| if i != n { Some(e) } else { None })
                    .collect::<Vec<_>>(),
            )
        }) {
            part2 += 1;
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (2, 4));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
