use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 15 } else { 141 };
const S: i32 = SIZE as i32;
const SAVE: i32 = if cfg!(test) { 50 } else { 100 };

type Point = (i32, i32);

fn manathan(a: Point, b: Point) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

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

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn next(&self, (x, y): Point) -> Point {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    fn get_deltas(&self) -> [Point; 41] {
        // points at Manhattan distance 20, on the half-diamond facing this direction
        match self {
            Direction::Up => std::array::from_fn(|i| {
                let dx = i as i32 - 20;
                (dx, -(20 - dx.abs()))
            }),
            Direction::Right => std::array::from_fn(|i| {
                let dy = i as i32 - 20;
                (20 - dy.abs(), dy)
            }),
            Direction::Down => std::array::from_fn(|i| {
                let dx = i as i32 - 20;
                (dx, 20 - dx.abs())
            }),
            Direction::Left => std::array::from_fn(|i| {
                let dy = i as i32 - 20;
                (-(20 - dy.abs()), dy)
            }),
        }
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut racetrack = [[i32::MIN; SIZE]; SIZE];
    let mut pos = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, &c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => racetrack[y][x] = i32::MAX,
                b'S' => {
                    racetrack[y][x] = 0;
                    pos = (x as i32, y as i32)
                }
                b'E' => end = (x as i32, y as i32),
                b'.' => (),
                _ => panic!(),
            }
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;
    let mut time = 0;
    let mut direction = Direction::new();
    let mut points = vec![];

    while pos != end {
        points.push((pos, time));
        time += 1;

        for d in [direction, direction.turn_left(), direction.turn_right()] {
            let next = d.next(pos);
            let n = (next.0 as usize, next.1 as usize);

            if racetrack[n.1][n.0] != i32::MAX {
                racetrack[n.1][n.0] = time;
                direction = d;
                pos = next;
                break;
            }
        }

        // add new points
        for (dx, dy) in direction.get_deltas() {
            let p = (pos.0 + dx, pos.1 + dy);

            if p.0 < 0 || p.1 < 0 || p.0 >= S || p.1 >= S {
                continue;
            }

            let race = racetrack[p.1 as usize][p.0 as usize];

            if race == i32::MAX || race == i32::MIN {
                continue;
            }

            points.push((p, race));
        }

        // for all points remove points that are too far and compute saved
        points = points
            .into_iter()
            .filter_map(|(p, race)| {
                let distance = manathan(p, pos);

                if distance > 20 {
                    None
                } else {
                    if distance > 1 {
                        let saved = time - race - distance as i32;

                        if saved >= SAVE {
                            if distance == 2 {
                                p1 += 1;
                            }
                            p2 += 1;
                        }
                    }
                    Some((p, race))
                }
            })
            .collect();
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1, 285));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
