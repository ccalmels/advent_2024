use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn new() -> Self {
        Direction::Up
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    fn to_mask(&self) -> u8 {
        match self {
            Direction::Up => 0b_0001,
            Direction::Right => 0b_0010,
            Direction::Down => 0b_0100,
            Direction::Left => 0b_1000,
        }
    }
}

#[test]
fn check_direction() {
    let up: u8 = Direction::Up.to_mask();

    assert_eq!(up, 0b_0001);
    assert_eq!(Direction::Up.turn_right(), Direction::Right);
}

struct Map {
    grid: Vec<Vec<u8>>,
    w: i32,
    h: i32,
}

impl Map {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let w = grid[0].len() as i32;
        let h = grid.len() as i32;

        Map { grid, w, h }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            None
        } else {
            Some(self.grid[y as usize][x as usize])
        }
    }

    fn set(&mut self, (x, y): (i32, i32), v: u8) {
        self.grid[y as usize][x as usize] = v;
    }
}

#[test]
fn check_map() {
    let grid = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let mut map = Map::new(grid);

    assert_eq!(map.get((-1, 0)), None);
    assert_eq!(map.get((0, 3)), None);
    assert_eq!(map.get((1, 1)), Some(4));

    map.set((1, 1), 99);

    assert_eq!(map.get((1, 1)), Some(99));
}

fn check_loop(
    map: &Map,
    blocker: (i32, i32),
    mut guard: (i32, i32),
    mut direction: Direction,
) -> bool {
    let mut dirs: HashMap<(i32, i32), u8> = HashMap::new();

    loop {
        let next_position = direction.next(guard);

        if let Some(v) = map.get(next_position) {
            if v == b'#' || next_position == blocker {
                direction = direction.turn_right();
            } else {
                let hentry = dirs.entry(next_position).or_default();
                let m = direction.to_mask();

                if *hentry & m == m {
                    return true;
                } else {
                    *hentry |= m;
                }

                guard = next_position;
            }
        } else {
            break;
        }
    }
    false
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = vec![];
    let mut guard = (0i32, 0i32);
    let mut direction = Direction::new();

    for line in lines {
        let line = line.unwrap();

        let index = line.as_bytes().iter().position(|&v| v == b'^');

        if let Some(index) = index {
            guard = (index as i32, grid.len() as i32);
        }

        grid.push(line.as_bytes().to_owned());
    }

    let mut map = Map::new(grid);
    let mut blocks: Vec<((i32, i32), (i32, i32), Direction)> = vec![];

    loop {
        let next_position = direction.next(guard);

        if let Some(v) = map.get(next_position) {
            if v == b'#' {
                direction = direction.turn_right();
            } else {
                if v == b'.' {
                    blocks.push((next_position, guard, direction.turn_right()));

                    map.set(next_position, b'X');
                }

                guard = next_position;
            }
        } else {
            break;
        }
    }

    (
        blocks.len() + 1,
        blocks
            .into_par_iter()
            .filter(|&(blocker, guard, direction)| check_loop(&map, blocker, guard, direction))
            .count(),
    )
}

#[test]
fn check() {
    const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (41, 6));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
