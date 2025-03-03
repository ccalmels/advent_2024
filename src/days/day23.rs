use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

fn resolve<T>(lines: Lines<T>) -> (usize, String)
where
    T: BufRead,
{
    let mut connections: HashMap<u16, HashSet<u16>> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let bytes = line.as_bytes();

        let a: u16 = ((bytes[0] as u16) << 8) | bytes[1] as u16;
        let b: u16 = ((bytes[3] as u16) << 8) | bytes[4] as u16;

        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    const T: u16 = b't' as u16;
    let mut p1 = 0;

    let mut lans: Vec<HashSet<u16>> = vec![];

    for (k, v) in &connections {
        for i in v {
            if let Some(set) = connections.get(i) {
                let inter = set.intersection(v).collect::<Vec<_>>();

                if k >> 8 == T || i >> 8 == T {
                    p1 += inter.len();
                } else {
                    for j in inter {
                        if j >> 8 == T {
                            p1 += 1;
                        }
                    }
                }
            }
        }

        let mut to_add = vec![HashSet::new()];

        for l in &mut lans {
            if l.is_subset(v) {
                to_add.push(l.clone());
            }
        }

        for mut add in to_add {
            add.insert(*k);
            lans.push(add);
        }
    }

    let mut max = 0;
    let mut max_index = 0;

    for (index, lan) in lans.iter().enumerate() {
        if lan.len() > max {
            max_index = index;
            max = lan.len();
        }
    }

    let mut res: Vec<u16> = lans[max_index].clone().into_iter().collect();
    res.sort_unstable();

    let mut p2 = String::from("");
    for c in res {
        let cs: [u8; 2] = [(c >> 8) as u8, (c & 0xff) as u8];

        p2 = format!(
            "{p2}{}{}",
            if p2.is_empty() { "" } else { "," },
            std::str::from_utf8(&cs).unwrap()
        );
    }

    (p1 / 6, p2)
}

#[test]
fn check() {
    const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    use std::io::Cursor;

    assert_eq!(
        resolve(Cursor::new(TEST).lines()),
        (7, String::from("co,de,ka,ta"))
    );
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
