use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 7 } else { 71 };
const S: i32 = SIZE as i32;
const FALLEN: usize = if cfg!(test) { 12 } else { 1024 };

type Point = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
struct Path(Point, usize);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(memory: &[[u8; SIZE]; SIZE]) -> usize {
    let mut heap = BinaryHeap::new();
    let mut distances = [[usize::MAX; SIZE]; SIZE];
    const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    distances[0][0] = 0;
    heap.push(Path((0, 0), 0));

    while let Some(Path((x, y), distance)) = heap.pop() {
        if x == S - 1 && y == S - 1 {
            return distance;
        }

        for (dx, dy) in DIRS {
            let (nextx, nexty) = (x + dx, y + dy);

            if nextx < 0 || nexty < 0 || nextx >= S || nexty >= S {
                continue;
            }

            let (x, y) = (nextx as usize, nexty as usize);

            if memory[y][x] == b'#' {
                continue;
            }

            if distance + 1 < distances[y][x] {
                heap.push(Path((nextx, nexty), distance + 1));
                distances[y][x] = distance + 1;
            }
        }
    }

    usize::MAX
}

fn resolve<T>(lines: Lines<T>) -> (usize, String)
where
    T: BufRead,
{
    let mut memory = [[b'.'; SIZE]; SIZE];
    let mut p1 = 0;
    let mut p2 = String::from("");

    for (index, line) in lines.enumerate() {
        if FALLEN == index {
            p1 = dijkstra(&memory);
        }

        let line = line.unwrap();

        let numbers: Vec<usize> = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let (x, y) = (numbers[0], numbers[1]);

        memory[y][x] = b'#';

        if FALLEN < index && dijkstra(&memory) == usize::MAX {
            p2 = line;
            break;
        }
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    use std::io::Cursor;

    assert_eq!(
        resolve(Cursor::new(TEST).lines()),
        (22, String::from("6,1"))
    );
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
