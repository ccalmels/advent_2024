use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Xmas {
    xmas_index: usize,
    samx_index: usize,
}

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

impl Xmas {
    fn new() -> Self {
        Xmas {
            xmas_index: 0,
            samx_index: XMAS.len() - 1,
        }
    }

    #[cfg(test)]
    fn reset(&mut self) {
        self.xmas_index = 0;
        self.samx_index = XMAS.len() - 1;
    }

    fn consume(&mut self, c: u8) -> bool {
        let mut ret = false;

        if XMAS[self.xmas_index] == c {
            if self.xmas_index == XMAS.len() - 1 {
                self.xmas_index = 0;
                ret = true;
            } else {
                self.xmas_index += 1;
            }
        } else {
            self.xmas_index = 0;
            if XMAS[self.xmas_index] == c {
                self.xmas_index += 1;
            }
        }

        if XMAS[self.samx_index] == c {
            if self.samx_index == 0 {
                self.samx_index = XMAS.len() - 1;
                ret = true;
            } else {
                self.samx_index -= 1;
            }
        } else {
            self.samx_index = XMAS.len() - 1;
            if XMAS[self.samx_index] == c {
                self.samx_index -= 1;
            }
        }

        ret
    }
}

#[test]
fn check_xmas() {
    let mut xmas = Xmas::new();

    assert_eq!(
        [b'X', b'M', b'A', b'S']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        1
    );
    xmas.reset();

    assert_eq!(
        [b'X', b'X', b'M', b'A', b'S']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        1
    );
    xmas.reset();

    assert_eq!(
        [b'S', b'A', b'M', b'X']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        1
    );
    xmas.reset();

    assert_eq!(
        [b'X', b'A', b'M', b'X']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        0
    );
    xmas.reset();

    assert_eq!(
        [b'X', b'M', b'A', b'S', b'X', b'M', b'A', b'S']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        2
    );
    xmas.reset();

    assert_eq!(
        [b'X', b'M', b'A', b'S', b'S', b'A', b'M', b'X']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        2
    );

    xmas.reset();

    assert_eq!(
        [b'X', b'M', b'A', b'S', b'A', b'M', b'X']
            .iter()
            .filter(|&c| xmas.consume(*c))
            .count(),
        2
    );
}

fn resolve<T>(lines: Lines<T>) -> (usize, u32)
where
    T: BufRead,
{
    let mut grid: Vec<Vec<u8>> = vec![];

    for line in lines {
        let line = line.unwrap();

        grid.push(line.as_bytes().to_vec());
    }

    let h = grid.len();
    let w = grid[0].len();
    let mut part1 = 0;

    // horizontal
    for y in 0..h {
        let mut xmas = Xmas::new();

        part1 += (0..w).filter(|&idx| xmas.consume(grid[y][idx])).count();
    }

    // vertical
    for x in 0..w {
        let mut xmas = Xmas::new();

        part1 += (0..h).filter(|&idx| xmas.consume(grid[idx][x])).count();
    }

    // diagonal
    for x in 0..w {
        let mut xmas = Xmas::new();

        part1 += (0..(w - x))
            .filter(|&idx| xmas.consume(grid[idx][idx + x]))
            .count();
    }

    for y in 1..h {
        let mut xmas = Xmas::new();

        part1 += (0..(h - y))
            .filter(|&idx| xmas.consume(grid[idx + y][idx]))
            .count();
    }

    for x in 0..w {
        let mut xmas = Xmas::new();

        part1 += (0..x + 1)
            .filter(|&idx| xmas.consume(grid[idx][x - idx]))
            .count();
    }

    for y in 1..h {
        let mut xmas = Xmas::new();

        part1 += (0..h - y)
            .filter(|&idx| xmas.consume(grid[y + idx][w - 1 - idx]))
            .count();
    }

    let mut part2 = 0;

    for x in 1..w - 1 {
        for y in 1..h - 1 {
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
