extern crate itertools;
extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(1928));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(6334655979668));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(2858));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(6349492251099));
}

fn compute_checksum(fs: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..fs.len() {
        if fs[i] != -1 {
            sum += i * fs[i] as usize;
        }
    }
    sum as i64
}

#[derive(Eq, Hash, PartialEq)]
struct File {
    id: u32,
    start_idx: usize,
    size: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct EmptySlot {
    start_idx: usize,
    size: usize,
}

fn solve(lines: &Vec<String>, is_part_b: bool) -> i64 {
    let mut files: Vec<File> = vec![];
    let mut empty_slots: Vec<EmptySlot> = vec![];
    let mut file_id = 0;
    let mut pos = 0;
    let mut is_file = true;
    let mut final_array: Vec<i64> = vec![];

    // parse input and create data structures
    for s in lines[0].chars() {
        let n = s.to_digit(10).unwrap() as usize;
        if is_file {
            // create file
            if is_part_b {
                // handle only complete files for part b
                files.push(File {
                    id: file_id,
                    start_idx: pos,
                    size: n,
                });
                final_array.append(&mut vec![file_id as i64; n]);
                pos += n;
            } else {
                // treat files as of length 1 for part a
                for _ in 0..n {
                    final_array.push(file_id as i64);
                    files.push(File {
                        id: file_id,
                        start_idx: pos,
                        size: 1,
                    });
                    pos += 1;
                }
            }
            file_id += 1;
            is_file = false;
        } else {
            // create empty slot
            empty_slots.push(EmptySlot {
                start_idx: pos,
                size: n,
            });
            final_array.append(&mut vec![-1; n]);
            pos += n;
            is_file = true;
        }
    }

    // move files to empty slots
    for file_i in (0..files.len()).rev() {
        let file = &files[file_i];
        for space_i in 0..empty_slots.len() {
            let space = &mut empty_slots[space_i];
            if space.start_idx < file.start_idx && space.size >= file.size {
                for i in 0..file.size {
                    final_array[file.start_idx + i] = -1;
                    final_array[space.start_idx + i] = file.id as i64;
                }
                space.size -= file.size;
                space.start_idx += file.size;
                break; // move to next file
            }
        }
    }

    compute_checksum(&final_array)
}

fn part_a(lines: &Vec<String>) -> i64 {
    solve(lines, false)
}

fn part_b(lines: &Vec<String>) -> i64 {
    solve(lines, true)
}
