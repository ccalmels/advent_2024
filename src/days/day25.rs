use advent_2024::Paragrapher;
use std::io::{BufRead, Lines};

const W: usize = 5;
const H: i32 = 5;

fn resolve<T>(mut lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut locks = vec![];
    let mut keys = vec![];

    for paragraph in lines.split_paragraph(|l| l) {
        let mut v = [-1; W];
        let is_lock = &paragraph[0][0..1] == "#";

        for p in paragraph {
            let p = p.as_bytes();

            for i in 0..5 {
                if p[i] == b'#' {
                    v[i] += 1;
                };
            }
        }

        if is_lock {
            locks.push(v);
        } else {
            keys.push(v);
        }
    }

    let mut p1 = 0;

    for l in locks {
        for k in keys.iter() {
            let overlap = l.iter().zip(k.iter()).any(|(a, b)| a + b > H);

            p1 += if overlap { 0 } else { 1 };
        }
    }

    (p1, 0)
}

#[test]
fn check() {
    const TEST: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (3, 0));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
