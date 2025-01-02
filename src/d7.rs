extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(3749));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(7885693428401));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(11387));
    show_and_check(
        "B",
        false,
        part_b(&input_lines) as i64,
        Some(348360680516005),
    );
}

fn parse_input(lines: &Vec<String>) -> Vec<(u64, Vec<u64>)> {
    let mut result: Vec<(u64, Vec<u64>)> = vec![];
    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let key = parts[0].parse::<u64>().unwrap();
        let values = parts[1]
            .split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        result.push((key, values));
    }

    result
}

fn check_line(key: u64, value: Vec<u64>) -> bool {
    let mut values = value;
    values.reverse();

    let mut computed = vec![values.pop().unwrap()];
    while !values.is_empty() {
        let current = values.pop().unwrap();
        let a = computed
            .clone()
            .into_iter()
            .map(|x| x + current)
            .collect::<Vec<u64>>();
        let b = computed
            .clone()
            .into_iter()
            .map(|x| x * current)
            .collect::<Vec<u64>>();
        computed = a;
        computed.extend(b);
        computed = computed.into_iter().filter(|x| *x <= key).collect();
    }

    computed.contains(&key)
}

fn check_line_b(key: u64, value: Vec<u64>) -> bool {
    let mut values = value;
    values.reverse();

    let mut computed = vec![values.pop().unwrap()];
    while !values.is_empty() {
        let current = values.pop().unwrap();
        let a = computed
            .clone()
            .into_iter()
            .map(|x| x + current)
            .collect::<Vec<u64>>();
        let b = computed
            .clone()
            .into_iter()
            .map(|x| x * current)
            .collect::<Vec<u64>>();
        let c = computed
            .clone()
            .into_iter()
            .map(|x| x.to_string() + &current.to_string())
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        computed = a;
        computed.extend(b);
        computed.extend(c);
        computed = computed.into_iter().filter(|x| *x <= key).collect();
    }

    computed.contains(&key)
}

fn part_a(lines: &Vec<String>) -> u64 {
    let map = parse_input(lines);

    let res = map
        .into_iter()
        .filter(|(k, v)| check_line(*k, v.clone()))
        .map(|(k, _)| k)
        .reduce(|k1, k2| k1 + k2)
        .unwrap();

    res
}

fn part_b(lines: &Vec<String>) -> u64 {
    let map = parse_input(lines);

    let res = map
        .into_iter()
        .filter(|(k, v)| check_line_b(*k, v.clone()))
        .map(|(k, _)| k)
        .reduce(|k1, k2| k1 + k2)
        .unwrap();

    res
}
