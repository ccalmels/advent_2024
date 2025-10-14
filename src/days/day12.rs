use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 140 };
const S: i32 = SIZE as i32;

fn price(plots: &[((i32, i32), usize)]) -> (usize, usize) {
    let mut p1 = plots.len() * 4;
    let mut corners: HashMap<(i32, i32), u8> = HashMap::new();
    const DELTAS: [(i32, i32, u8); 4] = [
        (0, 0, 0b0001),
        (1, 0, 0b0010),
        (0, 1, 0b0100),
        (1, 1, 0b1000),
    ];

    for &(point, neighbors) in plots.iter() {
        p1 -= neighbors;

        for (dx, dy, mask) in DELTAS {
            let corner = (point.0 + dx, point.1 + dy);

            *corners.entry(corner).or_default() |= mask;
        }
    }

    let p2 = corners.values().fold(0, |acc, &mask| {
        acc + match mask {
            0b1110 | 0b1101 | 0b1011 | 0b0111 => 1,
            0b1001 | 0b0110 => 2,
            0b0001 | 0b0010 | 0b0100 | 0b1000 => 1,
            _ => 0,
        }
    });

    (p1, p2)
}

fn found_region_dfs(
    grid: &mut [[u8; SIZE]; SIZE],
    point: (i32, i32),
    garden: u8,
) -> Vec<((i32, i32), usize)> {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut stack = vec![point];
    let mut points = vec![];

    while let Some((x, y)) = stack.pop() {
        let mut count = 0;

        for (dx, dy) in DIRS {
            let p = (x + dx, y + dy);

            if p.0 < 0 || p.1 < 0 || p.0 >= S || p.1 >= S {
                continue;
            }

            let g = &mut grid[p.1 as usize][p.0 as usize];
            let check_garden = *g ^ garden;

            if check_garden == 1 << 5 {
                // same region but already checked
                count += 1;
            } else if check_garden == 0 {
                count += 1;

                *g |= 1 << 5;

                stack.push(p);
            }
        }
        points.push(((x, y), count));
    }

    points
}

#[test]
fn check_found_region_dfs() {
    let mut grid = [[0; SIZE]; SIZE];

    grid[0][0] = b'a';
    grid[0][1] = b'A';
    grid[0][2] = b'A';
    grid[1][1] = b'A';
    grid[2][2] = b'A';

    let r = found_region_dfs(&mut grid, (0, 0), b'A');

    assert_eq!(r, vec![((0, 0), 1), ((1, 0), 3), ((1, 1), 1), ((2, 0), 1)]);
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut regions: Vec<Vec<((i32, i32), usize)>> = vec![];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        grid[y].copy_from_slice(line.as_bytes());
    }

    for y in 0..SIZE {
        for x in 0..SIZE {
            let garden = grid[y][x];

            // check if its uppercase
            if (garden & (1 << 5)) == 0 {
                grid[y][x] |= 1 << 5;

                regions.push(found_region_dfs(&mut grid, (x as i32, y as i32), garden))
            }
        }
    }

    regions
        .into_par_iter()
        .fold(
            || (0, 0),
            |(p1, p2), r| {
                let price = price(&r);

                (p1 + r.len() * price.0, p2 + r.len() * price.1)
            },
        )
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
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
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1930, 1206));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
