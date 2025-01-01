extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use std::collections::HashMap;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(11));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(1889772));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(31));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(23228917));
}

fn parse(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();
    for line in lines {
        let chars: Vec<_> = line.split_ascii_whitespace().collect();
        lhs.push(chars[0].parse().unwrap());
        rhs.push(chars[1].parse().unwrap());
    }

    (lhs, rhs)
}

fn part_a(lines: &Vec<String>) -> i32 {
    let (mut lhs, mut rhs) = parse(lines);

    lhs.sort();
    rhs.sort();
    let mut sum = 0;
    for i in 0..lhs.len() {
        sum += lhs[i].max(rhs[i]) - lhs[i].min(rhs[i]);
    }

    sum
}

fn part_b(lines: &Vec<String>) -> i32 {
    let (lhs, rhs) = parse(lines);

    let mut d: HashMap<i32, i32> = HashMap::new();
    for r in rhs {
        *d.entry(r).or_insert(0) += 1;
    }

    let mut sum = 0;
    for l in lhs {
        sum += d.get(&l).or_else(|| Some(&0)).unwrap() * l;
    }

    sum
}
