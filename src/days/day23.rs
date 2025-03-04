use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

fn bron_kernbosch(
    connections: &HashMap<u16, HashSet<u16>>,
    r: HashSet<u16>,
    mut p: HashSet<u16>,
    mut x: HashSet<u16>,
    max_clique: &mut HashSet<u16>,
) {
    if p.is_empty() && x.is_empty() {
        if max_clique.len() < r.len() {
            *max_clique = r;
        }
        return;
    }

    for v in p.clone() {
        let mut r2 = r.clone();
        r2.insert(v);

        let neighbors = connections.get(&v).unwrap();
        let p2 = p.intersection(neighbors).cloned().collect();
        let x2 = x.intersection(neighbors).cloned().collect();

        bron_kernbosch(connections, r2, p2, x2, max_clique);

        p.remove(&v);
        x.insert(v);
    }
}

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

    const T: u16 = (b't' as u16) << 8;

    let mut p1 = 0;

    for (k, v) in &connections {
        for i in v {
            if i < k {
                continue;
            }

            if let Some(set) = connections.get(i) {
                for j in set.intersection(v) {
                    if j < i {
                        continue;
                    }

                    if (*k & 0xff00) == T || (*i & 0xff00) == T || (*j & 0xff00) == T {
                        p1 += 1;
                    }
                }
            }
        }
    }

    let p: HashSet<u16> = connections.keys().cloned().collect();
    let r = HashSet::new();
    let x = HashSet::new();
    let mut max_clique = HashSet::new();

    bron_kernbosch(&connections, r, p, x, &mut max_clique);

    let mut points: Vec<u16> = max_clique.into_iter().collect();

    points.sort_unstable();

    let mut p2 = String::from("");

    for p in points {
        p2 = format!(
            "{}{}{}{}",
            p2,
            if p2.is_empty() { "" } else { "," },
            (p >> 8) as u8 as char,
            (p & 0xff) as u8 as char
        );
    }

    (p1, p2)
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
