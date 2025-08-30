use curl::easy::Easy;
use std::cmp::{Eq, Ord, Ordering};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::time::Instant;

pub struct Paragraph<'a, T, F, O>
where
    F: Fn(String) -> O,
{
    lines: &'a mut Lines<T>,
    transform: F,
}

pub trait Paragrapher<T, F, O>
where
    F: Fn(String) -> O,
{
    fn split_paragraph(&mut self, transfom: F) -> Paragraph<'_, T, F, O>;
}

impl<T, F, O> Paragrapher<T, F, O> for Lines<T>
where
    F: Fn(String) -> O,
{
    fn split_paragraph(&mut self, transform: F) -> Paragraph<'_, T, F, O> {
        Paragraph {
            lines: self,
            transform,
        }
    }
}

impl<T, F, O> Iterator for Paragraph<'_, T, F, O>
where
    T: BufRead,
    F: Fn(String) -> O,
{
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = vec![];

        for line in self.lines.by_ref() {
            let line = line.unwrap();

            if line.is_empty() {
                return Some(p);
            }

            p.push((self.transform)(line));
        }

        if p.is_empty() {
            None
        } else {
            Some(p)
        }
    }
}

#[derive(Eq)]
pub struct Day {
    day_filename: &'static str,
    resolve: fn(Lines<BufReader<File>>) -> (String, String),
}

impl Day {
    pub const fn new(
        day_filename: &'static str,
        resolve: fn(Lines<BufReader<File>>) -> (String, String),
    ) -> Self {
        Day {
            day_filename,
            resolve,
        }
    }

    fn print(&self) {
        let start = Instant::now();
        let (day_number, part1, part2) = self.resolve();
        let duration = start.elapsed();

        println!("day{day_number:0>2}: part1: {part1:20} part2: {part2:20} in {duration:?}");
    }

    fn parse_number(&self) -> u32 {
        self.day_filename
            .replace(|c: char| !c.is_ascii_digit(), "")
            .parse::<u32>()
            .unwrap()
    }

    fn resolve(&self) -> (u32, String, String) {
        let day_number = self.parse_number();
        let (part1, part2) = (self.resolve)(read_lines(day_number).unwrap());
        (day_number, part1, part2)
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.day_filename == other.day_filename
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day_filename.cmp(other.day_filename)
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_lines(day_number: u32) -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename = format!("./inputs/{day_number:0>2}.txt");
    let path = Path::new(&filename);

    if !path.exists() {
        println!("downloading input for day {day_number}");

        let session = env::var("AOC_SESSION").expect("AOC_SESSION not set");
        let mut file = File::create(path)?;
        let mut handle = Easy::new();

        handle.cookie(&format!("session={session}"))?;
        handle.url(&format!(
            "https://adventofcode.com/2024/day/{day_number}/input"
        ))?;

        handle.write_function(move |data| Ok(file.write(data).unwrap()))?;
        handle.perform()?;
    }

    let file = File::open(path)?;

    Ok(io::BufReader::new(file).lines())
}

fn resolve_all() {
    let mut days: Vec<&'static Day> = inventory::iter::<Day>.into_iter().collect();

    days.sort_unstable();

    days.iter().for_each(|d| d.print());
}

fn resolve_one(day_number: u32) {
    inventory::iter::<Day>
        .into_iter()
        .find(|d| d.parse_number() == day_number)
        .unwrap()
        .print();
}

pub fn resolve(days: &[u32]) {
    let start = Instant::now();

    if days.is_empty() {
        resolve_all();
    } else {
        days.iter().for_each(|&d| resolve_one(d));
    }

    let duration = start.elapsed();

    println!("All done in {duration:?}");
}

inventory::collect!(Day);
