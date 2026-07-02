use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

    fn forward(&self, p: Point) -> Point {
        match self {
            Direction::Up => (p.0, p.1 - 1),
            Direction::Right => (p.0 + 1, p.1),
            Direction::Down => (p.0, p.1 + 1),
            Direction::Left => (p.0 - 1, p.1),
        }
    }

    fn backward(&self, p: Point) -> Point {
        match self {
            Direction::Up => (p.0, p.1 + 1),
            Direction::Right => (p.0 - 1, p.1),
            Direction::Down => (p.0, p.1 - 1),
            Direction::Left => (p.0 + 1, p.1),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    point: Point,
    direction: Direction,
    score: usize,
}

impl State {
    fn new(point: Point, direction: Direction, score: usize) -> Self {
        State {
            point,
            direction,
            score,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<F>(scores: &mut [[[usize; 4]; SIZE]; SIZE], starts: &[(Point, Direction)], step: F)
where
    F: Fn(Direction, Point) -> Point,
{
    let mut heap = BinaryHeap::new();

    for &(p, d) in starts {
        heap.push(State::new(p, d, 0));
        scores[p.1 as usize][p.0 as usize][d as usize] = 0;
    }

    while let Some(state) = heap.pop() {
        let next_positions = [
            State::new(state.point, state.direction.turn_left(), state.score + 1000),
            State::new(
                state.point,
                state.direction.turn_right(),
                state.score + 1000,
            ),
            State::new(
                step(state.direction, state.point),
                state.direction,
                state.score + 1,
            ),
        ];

        for next in next_positions {
            let current_score =
                scores[next.point.1 as usize][next.point.0 as usize][next.direction as usize];

            if current_score <= next.score {
                continue;
            }

            scores[next.point.1 as usize][next.point.0 as usize][next.direction as usize] =
                next.score;

            heap.push(next);
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut scores = [[[usize::MAX; 4]; SIZE]; SIZE];
    let mut scores2 = [[[usize::MAX; 4]; SIZE]; SIZE];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => {
                    scores[y][x] = [0, 0, 0, 0];
                    scores2[y][x] = [0, 0, 0, 0]
                }
                b'S' => start = (x as u16, y as u16),
                b'E' => end = (x as u16, y as u16),
                b'.' => (),
                _ => panic!(),
            }
        }
    }

    dijkstra(&mut scores, &[(start, Direction::Right)], |d, p| {
        d.forward(p)
    });

    let p1 = (0..4)
        .map(|d| scores[end.1 as usize][end.0 as usize][d as usize])
        .min()
        .unwrap();

    let ends = [
        (end, Direction::Up),
        (end, Direction::Right),
        (end, Direction::Down),
        (end, Direction::Left),
    ];
    dijkstra(&mut scores2, &ends, |d, p| d.backward(p));

    let mut p2 = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if scores[y][x][0] == 0 {
                continue;
            }
            for d in 0..4 {
                if scores[y][x][d].overflowing_add(scores2[y][x][d]).0 == p1 {
                    p2 += 1;
                    break;
                }
            }
        }
    }

    (p1, p2)
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
