use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 140 };

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = [[ 0u8; SIZE]; SIZE];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        grid[y].clone_from_slice(line.as_bytes());
    }

    let mut part1 = 0;

    // horyzontal
    for line in grid {
        for x in 0..(SIZE-3) {
            let slice = &line[x..x+4];

            if slice == b"XMAS" || slice == b"SAMX" {
                part1 += 1;
            }
        }
    }

    // vertical
    for x in 0..SIZE {
        for y in 0..(SIZE-3) {
            let s = [ grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x] ];

            if s == *b"XMAS" || s == *b"SAMX" {
                part1 += 1;
            }
        }
    }

    // diagonals
    for y in 0..(SIZE-3) {
        for x in 0..(SIZE-3) {
            // diagonal \
            let s = [ grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3] ];

            if s == *b"XMAS" || s == *b"SAMX" {
                part1 += 1;
            }

            // diagonal /
            let s = [ grid[y][x+3], grid[y+1][x+2], grid[y+2][x+1], grid[y+3][x] ];

            if s == *b"XMAS" || s == *b"SAMX" {
                part1 += 1;
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
