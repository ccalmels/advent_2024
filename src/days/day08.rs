use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

fn add_antinode(antinodes: &mut HashSet<(i32, i32)>, pos: (i32, i32), v: (i32, i32), k: i32) -> bool
{
    const S: i32 = if cfg!(test) { 12 } else { 50 };
    let antinode = (pos.0 + k * v.0, pos.1 + k * v.1);

    if antinode.0 < 0 || antinode.1 < 0 || antinode.0 >= S || antinode.1 >= S {
	false
    } else {
	antinodes.insert(antinode);
	true
    }
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes2: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();

        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c != b'.' {
                let p = (x as i32, y as i32);

                antennas.entry(c).or_default().push(p);
            }
        }
    }

    for (_, positions) in antennas.iter() {
        let len = positions.len();

        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }

                let v = (
                    positions[j].0 - positions[i].0,
                    positions[j].1 - positions[i].1,
                );

		add_antinode(&mut antinodes, positions[j], v, 1);

		let mut k = 1;
		loop {
		    if !add_antinode(&mut antinodes2, positions[j], v, k) {
			break;
		    }
		    k += 1;
		}
            }
        }
    }

    for v in antennas.values() {
	for &a in v {
	    antinodes2.insert(a);
	}
    }

    (antinodes.len(), antinodes2.len())
}

#[test]
fn check() {
    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (14, 34));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
