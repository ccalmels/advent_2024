use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, Lines};

fn checksum(file_id: usize, offset: usize, size: usize) -> usize {
    (2 * offset + size - 1) * size * file_id / 2
}

fn resolve_part1(disk: &mut [u8]) -> usize {
    let mut index = 0;
    let mut last_index = disk.len() - 1;
    let mut offset = 0;
    let mut part1 = 0;

    while index <= last_index {
        let size = disk[index] as usize;

        if index % 2 == 0 {
            // file
            let file_id = index / 2;

            part1 += checksum(file_id, offset, size);
            offset += size;
        } else {
            // empty
            let last_file_space = disk[last_index] as usize;
            let file_id = last_index / 2;

            if size < last_file_space {
                part1 += checksum(file_id, offset, size);

                offset += size;

                disk[last_index] = (last_file_space - size) as u8;
            } else {
                last_index -= 2;

                part1 += checksum(file_id, offset, last_file_space);

                offset += last_file_space;

                if size != last_file_space {
                    disk[index] = (size - last_file_space) as u8;
                    continue;
                }
            }
        }

        index += 1;
    }

    part1
}

fn resolve_part2(
    file_blocks: &[(usize, usize)],
    empty_blocks: &mut [BinaryHeap<Reverse<usize>>; 10],
) -> usize {
    let mut part2 = 0;

    for (file_id, &(offset, size)) in file_blocks.iter().enumerate().rev() {
        // find a better place
        let mut better_places = empty_blocks
            .iter()
            .enumerate()
            .filter_map(|(idx, bheap)| {
                if idx < size {
                    None
                } else if let Some(Reverse(v)) = bheap.peek() {
                    if *v < offset {
                        Some((idx, *v))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if better_places.is_empty() {
            part2 += checksum(file_id, offset, size);
        } else {
            // take the place that's more on the left
            better_places.sort_unstable_by_key(|k| k.1);

            let (empty_size, empty_offset) = better_places[0];

            // remove the blocks
            empty_blocks[empty_size].pop();

            if empty_size > size {
                // reinsert the remaining empty blocks
                let new_empty_size = empty_size - size;

                empty_blocks[new_empty_size].push(Reverse(empty_offset + size));
            }

            part2 += checksum(file_id, empty_offset, size);
        }
    }

    part2
}

fn resolve<T>(mut lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut disk = vec![];
    let mut empty_blocks = [const { BinaryHeap::new() }; 10];
    let mut file_blocks = vec![];
    let mut is_file = true;
    let mut offset = 0;

    for c in lines.next().unwrap().unwrap().as_bytes() {
        let size = c - b'0';

        disk.push(size);

        let size = size as usize;
        if is_file {
            file_blocks.push((offset, size));
        } else {
            empty_blocks[size].push(Reverse(offset));
        }

        is_file = !is_file;
        offset += size;
    }

    (
        resolve_part1(&mut disk),
        resolve_part2(&file_blocks, &mut empty_blocks),
    )
}

#[test]
fn check() {
    const TEST: &str = "2333133121414131402";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1928, 2858));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
