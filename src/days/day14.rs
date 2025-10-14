use regex::Regex;
use std::io::{BufRead, Lines};

const W: i32 = if cfg!(test) { 11 } else { 101 };
const H: i32 = if cfg!(test) { 7 } else { 103 };

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn new(pos: (i32, i32), velocity: (i32, i32)) -> Self {
        Robot { pos, velocity }
    }

    fn step(&mut self, steps: i32) {
        self.pos.0 = (self.pos.0 + steps * self.velocity.0).rem_euclid(W);
        self.pos.1 = (self.pos.1 + steps * self.velocity.1).rem_euclid(H);
    }
}

#[cfg(test)]
fn print_bathroom(robots: &[Robot]) {
    for y in 0..H {
        for x in 0..W {
            let mut found = false;

            for r in robots {
                if r.pos.0 == x && r.pos.1 == y {
                    found = true;
                    break;
                }
            }

            if found {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(not(test))]
fn print_bathroom(_robots: &[Robot]) {}

fn compute_safety(robots: &[Robot]) -> usize {
    let mut squares = [0usize; 4];

    for r in robots.iter() {
        let index = match r.pos.0.cmp(&(W / 2)) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => continue,
        } + match r.pos.1.cmp(&(H / 2)) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => continue,
        };

        squares[index] += 1;
    }

    squares.iter().product()
}

fn steps(robots: &mut [Robot], steps: i32) {
    for r in robots.iter_mut() {
        r.step(steps);
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, i32)
where
    T: BufRead,
{
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots: Vec<Robot> = lines
        .map(|line| {
            let line = line.unwrap();

            let caps = re.captures(&line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let vx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let vy = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            Robot::new((x, y), (vx, vy))
        })
        .collect();

    let mut min_safety = usize::MAX;
    let mut part2 = 0;

    let mut robots_tree = robots.clone();
    let mut min_w = 0;

    for i in 0..W {
        steps(&mut robots_tree, 1);

        let safety = compute_safety(&robots_tree);

        if safety < min_safety {
            min_w = i + 1;
            min_safety = safety;
        }
    }

    robots_tree = robots.clone();
    steps(&mut robots_tree, min_w);

    for i in 0..H {
        steps(&mut robots_tree, W);

        let safety = compute_safety(&robots_tree);

        if safety < min_safety {
            part2 = min_w + (i + 1) * W;
            min_safety = safety;
        }
    }

    let mut robots = robots;

    steps(&mut robots, 100);

    print_bathroom(&robots);

    (compute_safety(&robots), part2)
}

#[test]
fn check() {
    const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()).0, 12);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
