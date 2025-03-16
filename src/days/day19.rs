use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn check_design_count<'a>(
    patterns: &[Vec<u8>],
    design: &'a [u8],
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if design.is_empty() {
        1
    } else if let Some(&n) = cache.get(design) {
        n
    } else {
        let ret = patterns.iter().fold(0, |acc, p| {
            let plen = p.len();

            if design.len() < plen || *p != design[0..plen] {
                acc
            } else {
                acc + check_design_count(patterns, &design[plen..], cache)
            }
        });

        cache.insert(design, ret);

        ret
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut patterns: Vec<Vec<u8>> = vec![];
    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        if patterns.is_empty() {
            patterns = line
                .split(&[',', ' '])
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.as_bytes().to_vec())
                    }
                })
                .collect();
        } else {
            let n = check_design_count(&patterns, line.as_bytes(), &mut HashMap::new());

            p1 += if n > 0 { 1 } else { 0 };
            p2 += n;
        }
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (6, 16));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
