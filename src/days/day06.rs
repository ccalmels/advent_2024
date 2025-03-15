use rayon::prelude::*;
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 130 };
const S: i32 = SIZE as i32;

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

    fn mask(&self) -> u8 {
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
    let up: u8 = Direction::Up.mask();

    assert_eq!(up, 0b_0001);
    assert_eq!(Direction::Up.turn_right(), Direction::Right);
}

struct Map {
    grid: [[u8; SIZE]; SIZE],
}

impl Map {
    fn new(grid: [[u8; SIZE]; SIZE]) -> Self {
        Map { grid }
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<u8> {
        if x < 0 || y < 0 || x >= S || y >= S {
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
    let grid = [[4; SIZE]; SIZE];
    let mut map = Map::new(grid);

    assert_eq!(map.get((-1, 0)), None);
    assert_eq!(map.get((0, S)), None);
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
    let mut dirs: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    loop {
        let next_position = direction.next(guard);

        if let Some(v) = map.get(next_position) {
            if v == b'#' || next_position == blocker {
                direction = direction.turn_right();
            } else {
                let entry = &mut dirs[next_position.1 as usize][next_position.0 as usize];
                let m = direction.mask();

                if *entry & m == m {
                    return true;
                } else {
                    *entry |= m;
                }

                guard = next_position;
            }
        } else {
            break;
        }
    }
    false
}

struct PossibleBlock {
    block_position: (i32, i32),
    position: (i32, i32),
    direction: Direction,
}

impl PossibleBlock {
    fn new(block_position: (i32, i32), position: (i32, i32), direction: Direction) -> Self {
        PossibleBlock {
            block_position,
            position,
            direction,
        }
    }

    fn check_loop(&self, map: &Map) -> bool {
        check_loop(map, self.block_position, self.position, self.direction)
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = [[0; SIZE]; SIZE];
    let mut guard = (0i32, 0i32);
    let mut direction = Direction::new();

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c == b'^' {
                guard = (x as i32, y as i32);
            }
            grid[y][x] = c;
        }
    }

    let mut map = Map::new(grid);
    let mut blocks: Vec<PossibleBlock> = vec![];

    loop {
        let next_position = direction.next(guard);

        if let Some(v) = map.get(next_position) {
            if v == b'#' {
                direction = direction.turn_right();
            } else {
                if v == b'.' {
                    blocks.push(PossibleBlock::new(
                        next_position,
                        guard,
                        direction.turn_right(),
                    ));

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
            .filter(|block| block.check_loop(&map))
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
