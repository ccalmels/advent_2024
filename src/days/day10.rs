use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 8 } else { 53 };
const S: i32 = SIZE as i32;
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn trailheads(zero: &(i32, i32), grid: &[[u8; SIZE]; SIZE]) -> (usize, usize) {
    let mut stack = vec![(zero.0, zero.1, 0)];
    let mut nines = [[false; SIZE]; SIZE];
    let (mut p1, mut p2) = (0, 0);

    while let Some((x, y, h)) = stack.pop() {
        for elem in DIRS.iter().filter_map(|(dx, dy)| {
            let (next_x, next_y) = (x + dx, y + dy);

            if next_x < 0 || next_y < 0 || next_x >= S || next_y >= S {
                None
            } else {
                let next_h = grid[next_y as usize][next_x as usize];

                if next_h == h + 1 {
                    if next_h == 9 {
                        p2 += 1;
                        if !nines[next_y as usize][next_x as usize] {
                            nines[next_y as usize][next_x as usize] = true;
                            p1 += 1;
                        }
                        None
                    } else {
                        Some((next_x, next_y, next_h))
                    }
                } else {
                    None
                }
            }
        }) {
            stack.push(elem);
        }
    }

    (p1, p2)
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = [[0u8; SIZE]; SIZE];
    let mut zeroes = vec![];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        line.as_bytes().iter().enumerate().for_each(|(x, &d)| {
            if d != b'.' {
                grid[y][x] = d - b'0';

                if d == b'0' {
                    zeroes.push((x as i32, y as i32));
                }
            }
        });
    }

    zeroes.into_iter().fold((0, 0), |(part1, part2), zero| {
        let (p1, p2) = trailheads(&zero, &grid);

        (part1 + p1, part2 + p2)
    })
}

#[test]
fn check() {
    const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    const TEST1: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const TEST2: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const TEST3: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const TEST4: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
    const TEST5: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const TEST6: &str = "012345
123456
234567
345678
4.6789
56789.";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, 2);
    assert_eq!(resolve(Cursor::new(TEST2).lines()).0, 4);
    assert_eq!(resolve(Cursor::new(TEST3).lines()).0, 3);
    assert_eq!(resolve(Cursor::new(TEST4).lines()).1, 3);
    assert_eq!(resolve(Cursor::new(TEST5).lines()).1, 13);
    assert_eq!(resolve(Cursor::new(TEST6).lines()).1, 227);

    assert_eq!(resolve(Cursor::new(TEST).lines()), (36, 81));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
