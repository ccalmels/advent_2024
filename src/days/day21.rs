use std::collections::HashMap;
use std::io::{BufRead, Lines};

type Point = (i32, i32);

struct Pad {
    get_point: fn(u8) -> Point,
    initial: Point,
    dead: Point,
}

const DIRECTIONAL: Pad = Pad {
    get_point: |key| match key {
        b'^' => (1, 0),
        b'A' => (2, 0),
        b'<' => (0, 1),
        b'v' => (1, 1),
        b'>' => (2, 1),
        _ => panic!(),
    },
    initial: (2, 0),
    dead: (0, 0),
};

const NUMERICAL: Pad = Pad {
    get_point: |key| match key {
        b'7' => (0, 0),
        b'8' => (1, 0),
        b'9' => (2, 0),
        b'4' => (0, 1),
        b'5' => (1, 1),
        b'6' => (2, 1),
        b'1' => (0, 2),
        b'2' => (1, 2),
        b'3' => (2, 2),
        b'0' => (1, 3),
        b'A' => (2, 3),
        _ => panic!(),
    },
    initial: (2, 3),
    dead: (0, 3),
};

struct Keypad {
    position: Point,
    pad: &'static Pad,
}

impl Keypad {
    fn new(pad: &'static Pad) -> Self {
        let position = pad.initial;

        Keypad { position, pad }
    }

    fn numeric() -> Self {
        Keypad::new(&NUMERICAL)
    }

    fn direction() -> Self {
        Keypad::new(&DIRECTIONAL)
    }

    fn click(&mut self, key: u8) -> Vec<Vec<u8>> {
        let mut ret = vec![];
        let next = (self.pad.get_point)(key);

        let dy = next.1 - self.position.1;
        let dx = next.0 - self.position.0;
        let dxdirection = if dx.is_positive() { b'>' } else { b'<' };
        let dxabs = dx.unsigned_abs() as usize;
        let dydirection = if dy.is_positive() { b'v' } else { b'^' };
        let dyabs = dy.unsigned_abs() as usize;

        // horizontal first
        if dxabs != 0 && (self.position.0 + dx, self.position.1) != self.pad.dead {
            let mut horizontal = vec![dxdirection; dxabs];

            horizontal.resize(dxabs + dyabs, dydirection);
            horizontal.push(b'A');

            ret.push(horizontal);
        }

        // vertical second
        if dyabs != 0 && (self.position.0, self.position.1 + dy) != self.pad.dead {
            let mut vertical = vec![dydirection; dyabs];

            vertical.resize(dyabs + dxabs, dxdirection);
            vertical.push(b'A');

            ret.push(vertical);
        }

        if ret.is_empty() {
            ret.push(vec![b'A']);
        }

        self.position = next;

        ret
    }
}

#[test]
fn check_keypad() {
    assert_eq!(Keypad::direction().click(b'<'), [b"v<<A"]);
    assert_eq!(Keypad::direction().click(b'v'), [b"<vA", b"v<A"]);

    let mut n = Keypad::numeric();

    assert_eq!(n.click(b'0'), [b"<A"]);
    assert_eq!(n.click(b'2'), [b"^A"]);
    assert_eq!(n.click(b'9'), [b">^^A", b"^^>A"]);
    assert_eq!(n.click(b'A'), [b"vvvA"]);
}

fn get_complexity(caches: &mut [HashMap<Vec<u8>, usize>], level: usize, keys: &[u8]) -> usize {
    if level == 0 {
        return keys.len();
    }

    if let Some(&complexity) = caches[level - 1].get(keys) {
        return complexity;
    }

    let mut d = Keypad::direction();
    let complexity = keys.iter().fold(0, |c, &key| {
        let mut res = d
            .click(key)
            .into_iter()
            .map(|seq| get_complexity(caches, level - 1, &seq))
            .collect::<Vec<_>>();

        res.sort_unstable();

        c + res[0]
    });

    caches[level - 1].insert(keys.to_vec(), complexity);

    complexity
}

#[test]
fn check_complexity() {
    let mut caches = vec![HashMap::new(); 3];

    assert_eq!(get_complexity(&mut caches, 0, b"v<<A"), 4);
    assert_eq!(get_complexity(&mut caches, 1, b"v<<A"), 10);

    assert_eq!(get_complexity(&mut caches, 2, b"<<^^A"), 23);
    assert_eq!(get_complexity(&mut caches, 2, b"^^<<A"), 27);

    assert_eq!(get_complexity(&mut caches, 3, b"^^<<A"), 65);
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut numeric = Keypad::numeric();
    let mut caches = vec![HashMap::new(); 25];

    lines.fold((0, 0), |(p1, p2), line| {
        let line = line.unwrap();
        let numeric_part = line[0..(line.len() - 1)].parse::<usize>().unwrap();

        line.as_bytes().iter().fold((p1, p2), |(c1, c2), &c| {
            let seqs = numeric.click(c);

            let mut complexities_2 = seqs
                .iter()
                .map(|seq| get_complexity(&mut caches, 2, seq))
                .collect::<Vec<_>>();

            complexities_2.sort_unstable();

            let mut complexities_25 = seqs
                .iter()
                .map(|seq| get_complexity(&mut caches, 25, seq))
                .collect::<Vec<_>>();

            complexities_25.sort_unstable();

            (
                c1 + complexities_2[0] * numeric_part,
                c2 + complexities_25[0] * numeric_part,
            )
        })
    })
}

#[test]
fn check() {
    const TEST: &str = "029A
980A
179A
456A
379A";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()).0, 126384);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
