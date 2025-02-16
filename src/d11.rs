extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use num::pow;
use std::collections::HashMap;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines), Some(55312));
    show_and_check("A", false, part_a(&input_lines), Some(222461));

    show_and_check("B", false, part_b(&input_lines), Some(264350935776416));
}

fn apply_rules(n: u64) -> Vec<u64> {
    let n_length = n.checked_ilog10().unwrap_or(0) + 1;
    if n == 0 {
        vec![1]
    } else if n_length % 2 == 0 {
        let div = pow(10u64, (n_length / 2) as usize);
        let upper = n / div;
        let lower = n % div;
        vec![upper, lower]
    } else {
        vec![n * 2024]
    }
}

fn n_terms(n: u64, iter: u32, cache: &mut HashMap<(u64, u32), u128>) -> u128 {
    if iter == 0 {
        1
    } else if cache.contains_key(&(n, iter)) {
        *cache.get(&(n, iter)).unwrap()
    } else {
        let res = apply_rules(n)
            .iter()
            .map(|i| n_terms(*i, iter - 1, cache))
            .sum();

        cache.insert((n, iter), res);
        res
    }
}

fn solve_b(lines: &Vec<String>, n_iter: u8) -> u128 {
    // build cache first. This works as all numbers are reduced to single digit numbers at some point
    let mut cache: HashMap<(u64, u32), u128> = HashMap::new();
    for i in 0..n_iter {
        for n in 0..10 {
            n_terms(n, i as u32, &mut cache);
        }
    }

    lines[0]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|s| n_terms(s, n_iter as u32, &mut cache))
        .sum::<u128>()
}

fn part_a(lines: &Vec<String>) -> i64 {
    solve_b(lines, 25) as i64
}

fn part_b(lines: &Vec<String>) -> i64 {
    solve_b(lines, 75) as i64
}
