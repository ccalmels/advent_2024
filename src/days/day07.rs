use itertools::Itertools;
use std::io::{BufRead, Lines};

fn number_of_digits(n: u64) -> u64 {
    let mut count = 1;
    let mut temp = n;

    while temp >= 1 {
        temp /= 10;
        count *= 10;
    }

    count
}

#[test]
fn check_number_of_digits() {
    assert_eq!(number_of_digits(123), 1000);
    assert_eq!(number_of_digits(8), 10);
    assert_eq!(number_of_digits(12345), 100000);
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn compute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => a * number_of_digits(b) + b,
        }
    }
}

#[test]
fn check_operation() {
    assert_eq!(Operation::Add.compute(2, 3), 5);
    assert_eq!(Operation::Multiply.compute(5, 7), 35);
    assert_eq!(Operation::Concatenate.compute(5, 73), 573);
}

fn check_all_operations(total: u64, values: &[u64], operations: &[Operation]) -> bool {
    std::iter::repeat(operations)
        .take(values.len() - 1)
        .multi_cartesian_product()
        .any(|x| {
            let mut t = values[0];

            for (operation, &value) in x.into_iter().zip(&values[1..]) {
                t = operation.compute(t, value);

                if t > total {
                    return false;
                }
            }

            total == t
        })
}

fn resolve<T>(lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    lines.fold((0, 0), |(p1, p2), line| {
        let splitted: Vec<u64> = line
            .unwrap()
            .split(&[':', ' '])
            .filter_map(|s| s.parse().ok())
            .collect();

        (
            if check_all_operations(
                splitted[0],
                &splitted[1..],
                &[Operation::Add, Operation::Multiply],
            ) {
                p1 + splitted[0]
            } else {
                p1
            },
            if check_all_operations(
                splitted[0],
                &splitted[1..],
                &[Operation::Add, Operation::Multiply, Operation::Concatenate],
            ) {
                p2 + splitted[0]
            } else {
                p2
            },
        )
    })
}

#[test]
fn check() {
    const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (3749, 11387));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
