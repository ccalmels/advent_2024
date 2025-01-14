use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn point_is_close((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> bool {
    ax.abs_diff(bx) + ay.abs_diff(by) == 1
}

fn space_is_close(plot: (usize, usize), space: &[(usize, usize)]) -> bool {
    space.iter().any(|&p| point_is_close(p, plot))
}

fn found_regions(plots: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut spaces: Vec<Vec<(usize, usize)>> = vec![];

    for &p in plots {
        let mut was_added = None;

        for i in (0..spaces.len()).rev() {
            if space_is_close(p, &spaces[i]) {
                if let Some(v) = was_added {
                    let plots = spaces.remove(v);
                    spaces[i].extend(plots);
                } else {
                    spaces[i].push(p);
                }
                was_added = Some(i);
            }
        }

        if was_added.is_none() {
            spaces.push(vec![p]);
        }
    }

    spaces
}

#[test]
fn check_found_regions() {
    let plots = vec![(0, 0), (2, 0), (1, 0)];

    assert_eq!(found_regions(&plots), vec![vec![(0, 0), (2, 0), (1, 0)]]);
}

fn price(plots: &[(usize, usize)]) -> (usize, usize) {
    let mut p1 = 0;
    let mut corners: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    const DELTAS: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

    for i in 0..plots.len() {
        let mut spaces = 4;

        for j in 0..plots.len() {
            if point_is_close(plots[i], plots[j]) {
                spaces -= 1;
                if spaces == 0 {
                    break;
                }
            }
        }

        p1 += spaces;

        for (dx, dy) in DELTAS {
            let corner = (plots[i].0 + dx, plots[i].1 + dy);

            corners.entry(corner).or_default().push(plots[i]);
        }
    }

    let p2 = corners.values().fold(0, |acc, neighbors| {
        acc + match neighbors.len() {
            4 => 0,
            3 => 1,
            2 => {
                if point_is_close(neighbors[0], neighbors[1]) {
                    0
                } else {
                    2
                }
            }
            1 => 1,
            _ => panic!(),
        }
    });

    (p1, p2)
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut plants: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, &garden) in line.as_bytes().iter().enumerate() {
            plants.entry(garden).or_default().push((x, y));
        }
    }

    plants.values().fold((0, 0), |(part1, part2), plots| {
        found_regions(plots)
            .iter()
            .fold((part1, part2), |(p1, p2), region| {
                let price = price(region);

                (p1 + region.len() * price.0, p2 + region.len() * price.1)
            })
    })
}

#[test]
fn check() {
    const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TEST1: &str = "AAX
AXA
AAA";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1930, 1206));
    assert_eq!(resolve(Cursor::new(TEST1).lines()), (120, 78));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
