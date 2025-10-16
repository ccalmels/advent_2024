use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 17 } else { 141 };

type Point = (u16, u16);

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

    fn next(&self, p: Point) -> Point {
        match self {
            Direction::Up => (p.0, p.1 - 1),
            Direction::Right => (p.0 + 1, p.1),
            Direction::Down => (p.0, p.1 + 1),
            Direction::Left => (p.0 - 1, p.1),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Path(Point, Direction, usize);

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

type Predecessors = Vec<(Point, Direction)>;

fn best_paths(
    scores: &mut [[[usize; 4]; SIZE]; SIZE],
    start: Point,
    d: Direction,
    end: Point,
) -> (usize, usize) {
    let mut pred: [[[Predecessors; 4]; SIZE]; SIZE] =
        core::array::from_fn(|_| core::array::from_fn(|_| core::array::from_fn(|_| Vec::new())));
    let mut heap = BinaryHeap::new();
    let mut v = vec![];

    heap.push(Path(start, d, 0));
    scores[start.1 as usize][start.0 as usize][d as usize] = 0;

    let mut p1 = usize::MAX;

    while let Some(Path(p, d, score)) = heap.pop() {
        let next = d.next(p);

        if next == end {
            p1 = score + 1;
            v.push((p, d));
            continue;
        }

        let nexts = [
            (p, d.turn_left(), score + 1000),
            (p, d.turn_right(), score + 1000),
            (next, d, score + 1),
        ];

        for (nextp, nextd, nextscore) in nexts {
            if nextscore > p1 {
                continue;
            }

            let scores_ref = &mut scores[nextp.1 as usize][nextp.0 as usize][nextd as usize];

            if nextscore > *scores_ref {
                continue;
            }

            let pred_ref = &mut pred[nextp.1 as usize][nextp.0 as usize][nextd as usize];

            if nextscore < *scores_ref {
                pred_ref.clear();
            }

            pred_ref.push((p, d));
            *scores_ref = nextscore;
            heap.push(Path(nextp, nextd, nextscore));
        }
    }

    let mut p2 = HashSet::from([end]);

    while let Some((p, d)) = v.pop() {
        p2.insert(p);

        for prevs in pred[p.1 as usize][p.0 as usize][d as usize].drain(..) {
            v.push(prevs);
        }
    }

    (p1, p2.len())
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut scores: [[[usize; 4]; SIZE]; SIZE] = [[[usize::MAX; 4]; SIZE]; SIZE];
    let mut position = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => scores[y][x] = [0, 0, 0, 0],
                b'S' => position = (x as u16, y as u16),
                b'E' => end = (x as u16, y as u16),
                b'.' => (),
                _ => panic!(),
            }
        }
    }

    best_paths(&mut scores, position, Direction::Right, end)
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
