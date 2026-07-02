use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 7 } else { 71 };
const FALLEN: usize = if cfg!(test) { 12 } else { 1024 };
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            Ordering::Less => self.parent[ra] = rb,
            Ordering::Greater => self.parent[rb] = ra,
            Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn idx(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRS.iter().filter_map(move |&(dx, dy)| {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx >= 0 && ny >= 0 && nx < SIZE as i32 && ny < SIZE as i32 {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}

fn bfs(blocked: &[[bool; SIZE]; SIZE]) -> usize {
    let mut visited = [[false; SIZE]; SIZE];
    let mut queue = VecDeque::new();

    visited[0][0] = true;
    queue.push_back((0usize, 0usize, 0usize));

    while let Some((x, y, dist)) = queue.pop_front() {
        if x == SIZE - 1 && y == SIZE - 1 {
            return dist;
        }
        for (nx, ny) in neighbors(x, y) {
            if !blocked[ny][nx] && !visited[ny][nx] {
                visited[ny][nx] = true;
                queue.push_back((nx, ny, dist + 1));
            }
        }
    }

    0
}

fn resolve<T>(lines: Lines<T>) -> (usize, String)
where
    T: BufRead,
{
    let bytes: Vec<(usize, usize)> = lines
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // p1: BFS after FALLEN bytes
    let mut blocked = [[false; SIZE]; SIZE];
    for &(x, y) in &bytes[..FALLEN] {
        blocked[y][x] = true;
    }
    let p1 = bfs(&blocked);

    // p2: union-find in reverse — block everything, then restore bytes one by one
    // until start and end are connected
    for &(x, y) in &bytes[FALLEN..] {
        blocked[y][x] = true;
    }

    let mut uf = UnionFind::new(SIZE * SIZE);

    for y in 0..SIZE {
        for x in 0..SIZE {
            if !blocked[y][x] {
                for (nx, ny) in neighbors(x, y) {
                    if !blocked[ny][nx] {
                        uf.union(idx(x, y), idx(nx, ny));
                    }
                }
            }
        }
    }

    let start = idx(0, 0);
    let end = idx(SIZE - 1, SIZE - 1);
    let mut p2 = String::new();

    for &(x, y) in bytes.iter().rev() {
        blocked[y][x] = false;

        for (nx, ny) in neighbors(x, y) {
            if !blocked[ny][nx] {
                uf.union(idx(x, y), idx(nx, ny));
            }
        }

        if uf.connected(start, end) {
            p2 = format!("{x},{y}");
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
    (solution.0.to_string(), solution.1)
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
