use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 50 };

fn get_direction(direction: u8) -> (i32, i32) {
    match direction {
        b'^' => (0, -1),
        b'>' => (1, 0),
        b'v' => (0, 1),
        b'<' => (-1, 0),
        _ => panic!(),
    }
}

fn walk(grid: &mut [[u8; SIZE]; SIZE], robot: &mut (i32, i32), direction: u8) {
    let direction = get_direction(direction);
    let (nextx, nexty) = (robot.0 + direction.0, robot.1 + direction.1);
    let mut nboxes = 0;

    loop {
        let (px, py) = (
            (nextx + nboxes * direction.0) as usize,
            (nexty + nboxes * direction.1) as usize,
        );

        match grid[py][px] {
            b'.' => {
                grid[py][px] = b'O';
                break;
            }
            b'O' => nboxes += 1,
            b'#' => return,
            _ => panic!(),
        }
    }

    *robot = (nextx, nexty);
    grid[nexty as usize][nextx as usize] = b'.';
}

#[cfg(test)]
fn print_grid(grid: &[[u8; SIZE]; SIZE], robot: &(i32, i32)) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if robot.0 as usize == x && robot.1 as usize == y {
                print!("@");
            } else {
                print!("{}", c as char);
            }
        }
        println!();
    }
    println!();
}

#[cfg(not(test))]
fn print_grid(_grid: &[[u8; SIZE]; SIZE], _robot: &(i32, i32)) {}

fn gps_boxes(grid: &[[u8; SIZE]; SIZE]) -> usize {
    grid.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(
            acc,
            |a, (x, &c)| {
                if c == b'O' {
                    a + y * 100 + x
                } else {
                    a
                }
            },
        )
    })
}

fn move_box_horizontaly(
    grid: &mut [[u8; 2 * SIZE]; SIZE],
    b: (i32, i32),
    direction: (i32, i32),
) -> bool {
    let g = grid[b.1 as usize][b.0 as usize];

    match g {
        b'#' => false,
        b'.' => true,
        _ => {
            if move_box_horizontaly(grid, (b.0 + direction.0, b.1), direction) {
                grid[b.1 as usize][(b.0 + direction.0) as usize] = g;
                true
            } else {
                false
            }
        }
    }
}

fn can_move_box_verticaly(
    grid: &[[u8; 2 * SIZE]; SIZE],
    b: (i32, i32),
    direction: (i32, i32),
) -> bool {
    match grid[b.1 as usize][b.0 as usize] {
        b'#' => false,
        b'.' => true,
        b'[' => {
            can_move_box_verticaly(grid, (b.0, b.1 + direction.1), direction)
                && can_move_box_verticaly(grid, (b.0 + 1, b.1 + direction.1), direction)
        }
        b']' => {
            can_move_box_verticaly(grid, (b.0, b.1 + direction.1), direction)
                && can_move_box_verticaly(grid, (b.0 - 1, b.1 + direction.1), direction)
        }
        _ => panic!(),
    }
}

fn move_box_verticaly(grid: &mut [[u8; 2 * SIZE]; SIZE], b: (i32, i32), direction: (i32, i32)) {
    match grid[b.1 as usize][b.0 as usize] {
        b'#' => (),
        b'.' => (),
        b'[' => {
            move_box_verticaly(grid, (b.0, b.1 + direction.1), direction);
            move_box_verticaly(grid, (b.0 + 1, b.1 + direction.1), direction);

            grid[b.1 as usize][b.0 as usize] = b'.';
            grid[b.1 as usize][(b.0 + 1) as usize] = b'.';
            grid[(b.1 + direction.1) as usize][b.0 as usize] = b'[';
            grid[(b.1 + direction.1) as usize][(b.0 + 1) as usize] = b']';
        }
        b']' => {
            move_box_verticaly(grid, (b.0, b.1 + direction.1), direction);
            move_box_verticaly(grid, (b.0 - 1, b.1 + direction.1), direction);

            grid[b.1 as usize][b.0 as usize] = b'.';
            grid[b.1 as usize][(b.0 - 1) as usize] = b'.';
            grid[(b.1 + direction.1) as usize][b.0 as usize] = b']';
            grid[(b.1 + direction.1) as usize][(b.0 - 1) as usize] = b'[';
        }
        _ => panic!(),
    }
}

fn walk2(grid: &mut [[u8; 2 * SIZE]; SIZE], robot: &mut (i32, i32), direction: u8) {
    let direction = get_direction(direction);
    let (nextx, nexty) = (robot.0 + direction.0, robot.1 + direction.1);

    if direction.1 == 0 {
        // horizontal
        if move_box_horizontaly(grid, (nextx, nexty), direction) {
            grid[nexty as usize][nextx as usize] = b'.';
            *robot = (nextx, nexty);
        }
    } else {
        // vertical
        if can_move_box_verticaly(grid, (nextx, nexty), direction) {
            move_box_verticaly(grid, (nextx, nexty), direction);
            *robot = (nextx, nexty);
        }
    }
}

#[cfg(test)]
fn print_grid2(grid: &[[u8; 2 * SIZE]; SIZE], robot: &(i32, i32)) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if robot.0 as usize == x && robot.1 as usize == y {
                print!("@");
            } else {
                print!("{}", c as char);
            }
        }
        println!();
    }
    println!();
}

#[cfg(not(test))]
fn print_grid2(_grid: &[[u8; 2 * SIZE]; SIZE], _robot: &(i32, i32)) {}

fn gps_boxes2(grid: &[[u8; 2 * SIZE]; SIZE]) -> usize {
    grid.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(
            acc,
            |a, (x, &c)| {
                if c == b'[' {
                    a + y * 100 + x
                } else {
                    a
                }
            },
        )
    })
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid: [[u8; SIZE]; SIZE] = [[b'.'; SIZE]; SIZE];
    let mut grid2: [[u8; 2 * SIZE]; SIZE] = [[b'.'; 2 * SIZE]; SIZE];
    let mut robot = (0, 0);
    let mut robot2 = (0, 0);
    let mut is_direction = false;

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            is_direction = true;
            continue;
        }

        if !is_direction {
            for (x, &c) in line.as_bytes().iter().enumerate() {
                if c == b'@' {
                    robot = (x as i32, y as i32);
                    robot2 = ((x * 2) as i32, y as i32);
                } else if c != b'.' {
                    grid[y][x] = c;
                    match c {
                        b'#' => {
                            grid2[y][2 * x] = b'#';
                            grid2[y][2 * x + 1] = b'#';
                        }
                        b'O' => {
                            grid2[y][2 * x] = b'[';
                            grid2[y][2 * x + 1] = b']';
                        }
                        _ => panic!(),
                    }
                }
            }
        } else {
            for &d in line.as_bytes().iter() {
                walk(&mut grid, &mut robot, d);
                walk2(&mut grid2, &mut robot2, d);
            }
        }
    }

    print_grid(&grid, &robot);
    print_grid2(&grid2, &robot2);

    (gps_boxes(&grid), gps_boxes2(&grid2))
}

#[test]
fn check() {
    const TEST1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const TEST2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    const TEST3: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, 2028);
    assert_eq!(resolve(Cursor::new(TEST2).lines()).1, 105 + 207 + 306);
    assert_eq!(resolve(Cursor::new(TEST3).lines()), (10092, 9021));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
