extern crate num;
extern crate regex;

use regex::Regex;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    // show_and_check("A", true, part_a(&test_lines) as i64, Some(161));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(179571322));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(48));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(103811193));
}

fn parse_and_multiply(s: &str) -> i64 {
    s.replace("mul(", "")
        .replace(")", "")
        .split(",")
        .map(|_t2| _t2.parse::<i64>().unwrap())
        .reduce(|a, b| a * b)
        .unwrap()
}

fn part_a(lines: &Vec<String>) -> i64 {
    let s = lines.join("");
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    re.find_iter(&s)
        .map(|c| c.as_str())
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|t| parse_and_multiply(t))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part_b(lines: &Vec<String>) -> i64 {
    let s = lines.join("");
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();

    let mut result = 0;
    let mut to_include = true;

    for s1 in re.find_iter(&s).map(|c| c.as_str()).collect::<Vec<&str>>() {
        if s1.starts_with("mul") && to_include {
            result += parse_and_multiply(s1);
        } else if s1 == "do()" {
            to_include = true;
        } else if s1 == "don't()" {
            to_include = false;
        } else {
            continue;
        }
    }

    result
}
