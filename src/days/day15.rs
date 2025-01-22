use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 50 };

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn up(&self) -> Self {
        Point(self.0, self.1.checked_sub(1).unwrap())
    }

    fn right(&self) -> Self {
        Point(self.0.checked_add(1).unwrap(), self.1)
    }

    fn down(&self) -> Self {
        Point(self.0, self.1.checked_add(1).unwrap())
    }

    fn left(&self) -> Self {
        Point(self.0.checked_sub(1).unwrap(), self.1)
    }

    fn move_to(&self, direction: u8) -> Self {
        match direction {
            b'^' => self.up(),
            b'>' => self.right(),
            b'v' => self.down(),
            b'<' => self.left(),
            _ => panic!(),
        }
    }
}

fn walk(grid: &mut [[u8; 2 * SIZE]; SIZE], robot: Point, direction: u8) -> Point {
    let mut next = robot;

    loop {
        next = next.move_to(direction);

        match grid[next.1][next.0] {
            b'.' => {
                let robot = robot.move_to(direction);
                grid[next.1][next.0] = b'O';
                grid[robot.1][robot.0] = b'.';
                return robot;
            }
            b'O' => (),
            b'#' => return robot,
            _ => panic!(),
        }
    }
}

fn move_box_horizontaly(grid: &mut [[u8; 2 * SIZE]; SIZE], p: Point, direction: u8) -> bool {
    let g = grid[p.1][p.0];

    match g {
        b'#' => false,
        b'.' => true,
        _ => {
            let p = p.move_to(direction);

            if move_box_horizontaly(grid, p, direction) {
                grid[p.1][p.0] = g;
                true
            } else {
                false
            }
        }
    }
}

fn can_move_box_verticaly(grid: &[[u8; 2 * SIZE]; SIZE], p: Point, direction: u8) -> bool {
    match grid[p.1][p.0] {
        b'#' => false,
        b'.' => true,
        b'[' => {
            let p = p.move_to(direction);

            can_move_box_verticaly(grid, p, direction)
                && can_move_box_verticaly(grid, p.right(), direction)
        }
        b']' => {
            let p = p.move_to(direction);

            can_move_box_verticaly(grid, p, direction)
                && can_move_box_verticaly(grid, p.left(), direction)
        }
        _ => panic!(),
    }
}

fn move_box_verticaly(grid: &mut [[u8; 2 * SIZE]; SIZE], p: Point, direction: u8) {
    match grid[p.1][p.0] {
        b'#' => (),
        b'.' => (),
        b'[' => {
            let next_left = p.move_to(direction);
            let next_right = next_left.right();
            let p2 = p.right();

            move_box_verticaly(grid, next_left, direction);
            move_box_verticaly(grid, next_right, direction);

            grid[p.1][p.0] = b'.';
            grid[p2.1][p2.0] = b'.';
            grid[next_left.1][next_left.0] = b'[';
            grid[next_right.1][next_right.0] = b']';
        }
        b']' => {
            let next_right = p.move_to(direction);
            let next_left = next_right.left();
            let p2 = p.left();

            move_box_verticaly(grid, next_right, direction);
            move_box_verticaly(grid, next_left, direction);

            grid[p.1][p.0] = b'.';
            grid[p2.1][p2.0] = b'.';
            grid[next_left.1][next_left.0] = b'[';
            grid[next_right.1][next_right.0] = b']';
        }
        _ => panic!(),
    }
}

fn walk2(grid: &mut [[u8; 2 * SIZE]; SIZE], robot: Point, direction: u8) -> Point {
    let next = robot.move_to(direction);

    if direction == b'<' || direction == b'>' {
        // horizontal
        if move_box_horizontaly(grid, next, direction) {
            grid[next.1][next.0] = b'.';
            return next;
        }
    } else {
        // vertical
        if can_move_box_verticaly(grid, next, direction) {
            move_box_verticaly(grid, next, direction);
            return next;
        }
    }

    robot
}

#[cfg(test)]
fn print_grid(grid: &[[u8; 2 * SIZE]; SIZE], robot: &Point) {
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if robot.0 == x && robot.1 == y {
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
fn print_grid(_grid: &[[u8; 2 * SIZE]; SIZE], _robot: &Point) {}

fn gps_boxes(grid: &[[u8; 2 * SIZE]; SIZE]) -> usize {
    grid.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter().enumerate().fold(acc, |a, (x, &c)| {
            if c == b'[' || c == b'O' {
                a + y * 100 + x
            } else {
                a
            }
        })
    })
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid: [[u8; 2 * SIZE]; SIZE] = [[b'.'; 2 * SIZE]; SIZE];
    let mut grid2: [[u8; 2 * SIZE]; SIZE] = [[b'.'; 2 * SIZE]; SIZE];
    let mut robot = Point(0, 0);
    let mut robot2 = Point(0, 0);
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
                    robot = Point(x, y);
                    robot2 = Point(x * 2, y);
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
                robot = walk(&mut grid, robot, d);
                robot2 = walk2(&mut grid2, robot2, d);
            }
        }
    }

    print_grid(&grid, &robot);
    print_grid(&grid2, &robot2);

    (gps_boxes(&grid), gps_boxes(&grid2))
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
