use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 17 } else { 141 };

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

type Point = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
struct Path(Point, Direction, usize, Vec<Point>);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn best_paths(grid: &[[u8; SIZE]; SIZE], start: Point, d: Direction, end: Point) -> (usize, usize) {
    let mut scores: [[[usize; 4]; SIZE]; SIZE] = [[[usize::MAX; 4]; SIZE]; SIZE];
    let mut heap = BinaryHeap::new();

    heap.push(Path(start, d, 0, vec![]));

    let mut p1 = usize::MAX;
    let mut p2 = HashSet::new();

    while let Some(Path(p, d, score, points)) = heap.pop() {
        let x = p.0 as usize;
        let y = p.1 as usize;

        scores[y][x][d as usize] = score;

        let left = d.turn_left();

        if scores[y][x][left as usize] > score + 1000 && score + 1000 < p1 {
            heap.push(Path(p, left, score + 1000, points.clone()));
        }

        let right = d.turn_right();

        if scores[y][x][right as usize] > score + 1000 && score + 1000 < p1 {
            heap.push(Path(p, right, score + 1000, points.clone()));
        }

        let next = match d {
            Direction::Up => (p.0, p.1 - 1),
            Direction::Right => (p.0 + 1, p.1),
            Direction::Down => (p.0, p.1 + 1),
            Direction::Left => (p.0 - 1, p.1),
        };

        let next_x = next.0 as usize;
        let next_y = next.1 as usize;

        if grid[next_y][next_x] == b'.' {
            if next == end {
                p1 = score;
                p2.extend(points);
            } else if scores[next_y][next_x][d as usize] > score && score + 1 < p1 {
                let mut points = points;
                points.push(p);

                heap.push(Path(next, d, score + 1, points));
            }
        }
    }

    (p1 + 1, p2.len() + 2)
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid: [[u8; SIZE]; SIZE] = [[b'.'; SIZE]; SIZE];
    let mut position = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => grid[y][x] = b'#',
                b'S' => position = (x as i32, y as i32),
                b'E' => end = (x as i32, y as i32),
                b'.' => (),
                _ => panic!(),
            }
        }
    }

    best_paths(&grid, position, Direction::Right, end)
}

#[test]
fn check() {
    const TEST1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const TEST2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()), (7036, 45));
    assert_eq!(resolve(Cursor::new(TEST2).lines()), (11048, 64));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
