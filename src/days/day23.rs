use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Lines};

// Different algos
//
// Bron-KernBosch
// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
//
// Algorithm 1 from:
// https://www.internetmathematicsjournal.com/article/1586-fast-algorithms-for-the-maximum-clique-problem-on-massive-graphs-with-applications-to-overlapping-community-detection.pdf
//
// Branch-and-Bound
// https://arxiv.org/html/2403.09742v1
//
// other resources:
// https://arxiv.org/pdf/1101.1266
fn best_clique(
    connections: &HashMap<u16, HashSet<u16>>,
    clique: HashSet<u16>,
    mut p: HashSet<u16>,
    best: &mut HashSet<u16>,
) {
    if clique.len() + p.len() <= best.len() {
        return;
    }

    if p.is_empty() {
        *best = clique;
        return;
    }

    for v in p.clone() {
        let mut new_clique = clique.clone();
        new_clique.insert(v);

        let neighbors = &connections[&v];
        let p2: HashSet<u16> = p.intersection(neighbors).copied().collect();

        best_clique(connections, new_clique, p2, best);

        p.remove(&v);

        if clique.len() + p.len() <= best.len() {
            return;
        }
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

    let p: HashSet<u16> = connections.keys().copied().collect();
    let mut max_clique = HashSet::new();

    best_clique(&connections, HashSet::new(), p, &mut max_clique);

    let mut points: Vec<u16> = Vec::from_iter(max_clique);

    points.sort_unstable();

    let p2 = points
        .iter()
        .map(|&p| format!("{}{}", (p >> 8) as u8 as char, (p & 0xff) as u8 as char))
        .collect::<Vec<_>>()
        .join(",");

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
    (solution.0.to_string(), solution.1)
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
