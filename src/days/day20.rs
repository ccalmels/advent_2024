use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 15 } else { 141 };
const S: i32 = SIZE as i32;
const SAVE: usize = if cfg!(test) { 50 } else { 100 };
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

type Point = (i32, i32);

fn bfs(
    racetrack: &[[u8; SIZE]; SIZE],
    start: (i32, i32),
    limit: usize,
) -> HashSet<(Point, usize)> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut distances = [[usize::MAX; SIZE]; SIZE];
    let mut points = HashSet::new();

    distances[start.1 as usize][start.0 as usize] = 0;

    while let Some((pos, distance)) = queue.pop_back() {
        //        println!("{pos:?} => {distance}");

        let distance = distance + 1;
        if distance > limit {
            continue;
        }

        for (dx, dy) in DIRS {
            let next = (pos.0 + dx, pos.1 + dy);

            if next == start {
                continue;
            }

            if next.0 < 0 || next.1 < 0 || next.0 >= S || next.1 >= S {
                continue;
            }

            let n = (next.0 as usize, next.1 as usize);
            let d = distances[n.1][n.0];

            if distance < d {
                distances[n.1][n.0] = distance;

                if racetrack[n.1][n.0] == b'.' {
                    if distance > 1 {
                        assert!(!points.contains(&(next, distance)));
                        points.insert((next, distance));
                    }
                }
                queue.push_front((next, distance));
            }
        }
    }

    points
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut racetrack = [[b'.'; SIZE]; SIZE];
    let mut courses = [[usize::MAX; SIZE]; SIZE];
    let mut pos = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, &c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => racetrack[y][x] = c,
                b'S' => pos = (x as i32, y as i32),
                b'E' => end = (x as i32, y as i32),
                b'.' => (),
                _ => panic!(),
            }
        }
    }

    let mut course = 0;
    courses[pos.1 as usize][pos.0 as usize] = 0;

    let mut p1 = 0;
    let mut p2 = 0;

    loop {
        course += 1;

        // get next position
        for (dx, dy) in DIRS {
            let next = (pos.0 + dx, pos.1 + dy);
            let n = (next.0 as usize, next.1 as usize);

            if racetrack[n.1][n.0] == b'.' {
                if courses[n.1][n.0] > course {
                    courses[n.1][n.0] = course;
                    pos = next;
                    break;
                }
            }
        }

        for (start, distance) in bfs(&racetrack, pos, 20) {
            let s = (start.0 as usize, start.1 as usize);
            let s = courses[s.1][s.0];
            let e = course;

            if s == usize::MAX {
                continue;
            }

            if e > s + distance {
                let save = e - s - distance;

                //println!("save: {save}/{distance} : {start:?}({s}) -> {pos:?}({e})");

                if save >= SAVE {
                    if distance == 2 {
                        p1 += 1;
                    }
                    p2 += 1;
                }
            }
        }

        if pos == end {
            break;
        }
    }

    //    println!("course: {course}");

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
