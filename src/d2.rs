extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(2));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(411));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(4));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(465));
}

fn parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut lhs: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let x: Vec<i32> = line
            .split_ascii_whitespace()
            .into_iter()
            .map(|c| c.parse().unwrap())
            .collect();
        lhs.push(x);
    }

    lhs
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut diff: Vec<i32> = Vec::new();
    for i in 1..report.len() {
        diff.push(report[i] - report[i - 1]);
    }

    let all_neg = diff.iter().all(|&x| x < 0);
    let all_pos = diff.iter().all(|&x| x > 0);
    let g0 = diff.iter().all(|&x| x.abs() >= 1);
    let lt = diff.iter().all(|&x| x.abs() <= 3);

    (all_neg || all_pos) && g0 && lt
}

fn part_a(lines: &Vec<String>) -> i32 {
    let levels = parse(lines);

    let mut sum = 0;
    for level in levels {
        if is_report_safe(&level) {
            sum += 1;
        }
    }
    sum
}

fn part_b(lines: &Vec<String>) -> i32 {
    let levels = parse(lines);

    let mut sum = 0;
    for level in levels {
        let mut is_safe = vec![is_report_safe(&level)];

        // remove each element and check if the report is safe
        for i in 0..level.len() {
            let mut new_level = level.clone();
            new_level.remove(i);
            is_safe.push(is_report_safe(&new_level));
        }

        if is_safe.iter().any(|&x| x) {
            sum += 1;
        }
    }
    sum
}
