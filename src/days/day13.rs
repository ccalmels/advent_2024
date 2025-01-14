use advent_2024::Paragrapher;
use regex::Regex;
use std::io::{BufRead, Lines};

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
}

impl Machine {
    fn new(ax: i64, ay: i64, bx: i64, by: i64) -> Self {
        Machine { ax, ay, bx, by }
    }

    fn presses(&self, (prizex, prizey): (i64, i64)) -> (i64, i64) {
        // A * ax + B * bx = prizex
        // A * ay + B * by = prizey
        //
        // A * ax * by + B * bx * by = prizex * by
        // A * ay * bx + B * by * bx = prizey * bx
        //
        // A * (ax * by - ay * bx) + 0 = prizex * by - prizey * bx
        //
        let numerator1 = prizex * self.by - prizey * self.bx;
        let numerator2 = numerator1 + 10000000000000 * (self.by - self.bx);
        let denominator = self.ax * self.by - self.ay * self.bx;

        (
            if numerator1.signum() == denominator.signum() && numerator1 % denominator == 0 {
                let a = numerator1 / denominator;
                let b = (prizex - a * self.ax) / self.bx;
                3 * a + b
            } else {
                0
            },
            if numerator2.signum() == denominator.signum() && numerator2 % denominator == 0 {
                let a = numerator2 / denominator;
                let b = (10000000000000 + prizex - a * self.ax) / self.bx;
                3 * a + b
            } else {
                0
            },
        )
    }
}

fn resolve<T>(mut lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let re_button = Regex::new(r"Button \w: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    lines
        .split_paragraph(|l| l)
        .map(|paragraph| {
            let caps = re_button.captures(&paragraph[0]).unwrap();
            let ax = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let ay = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let caps = re_button.captures(&paragraph[1]).unwrap();
            let bx = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let by = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let caps = re_prize.captures(&paragraph[2]).unwrap();
            let prizex = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let prizey = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();

            (Machine::new(ax, ay, bx, by), (prizex, prizey))
        })
        .fold((0, 0), |(p1, p2), (m, p)| {
            let presses = m.presses(p);

            (p1 + presses.0, p2 + presses.1)
        })
}

#[test]
fn check() {
    const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (480, 875318608908));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
