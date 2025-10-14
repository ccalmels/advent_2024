use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 140 };

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = [[0u8; SIZE]; SIZE];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        grid[y].clone_from_slice(line.as_bytes());
    }

    let mut part1 = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            // horizontal
            if x + 3 < SIZE {
                let horizontal = &grid[y][x..x + 4];

                if horizontal == b"XMAS" || horizontal == b"SAMX" {
                    part1 += 1;
                }
            }

            // vertical
            if y + 3 < SIZE {
                let vertical = [grid[y][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x]];

                if vertical == *b"XMAS" || vertical == *b"SAMX" {
                    part1 += 1;
                }
            }

            // diagonals
            if x + 3 < SIZE && y + 3 < SIZE {
                // diagonal \
                let antislash = [
                    grid[y][x],
                    grid[y + 1][x + 1],
                    grid[y + 2][x + 2],
                    grid[y + 3][x + 3],
                ];

                if antislash == *b"XMAS" || antislash == *b"SAMX" {
                    part1 += 1;
                }

                // diagonal /
                let slash = [
                    grid[y][x + 3],
                    grid[y + 1][x + 2],
                    grid[y + 2][x + 1],
                    grid[y + 3][x],
                ];

                if slash == *b"XMAS" || slash == *b"SAMX" {
                    part1 += 1;
                }
            }
        }
    }

    let mut part2 = 0;

    for x in 1..SIZE - 1 {
        for y in 1..SIZE - 1 {
            if grid[y][x] == b'A'
                && grid[y - 1][x - 1] + grid[y + 1][x + 1] == b'S' + b'M'
                && grid[y - 1][x + 1] + grid[y + 1][x - 1] == b'S' + b'M'
            {
                part2 += 1;
            }
        }
    }

    (part1, part2)
}

#[test]
fn check() {
    const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (18, 9));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
