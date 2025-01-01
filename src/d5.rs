extern crate num;
extern crate regex;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use std::collections::HashMap;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(143));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(5955));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(123));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(4030));
}

fn split_input(
    lines: &Vec<String>,
) -> (
    HashMap<i32, Vec<i32>>,
    HashMap<i32, Vec<i32>>,
    Vec<Vec<i32>>,
) {
    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages: Vec<Vec<i32>> = vec![];

    for line in lines {
        if line.contains("|") {
            let p = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v = ordering.get_mut(&p[0]);
            if v.is_none() {
                ordering.insert(p[0], vec![p[1]]);
            } else {
                v.unwrap().push(p[1]);
            }
        } else if line.contains(",") {
            let p = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            pages.push(p);
        } else {
            continue;
        }
    }

    let right_of = get_right_ordering(&ordering);

    (ordering, right_of, pages)
}

fn get_right_ordering(left_of: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut right_of: HashMap<i32, Vec<i32>> = HashMap::new();

    for (k, v) in left_of.into_iter() {
        for i in v {
            let r = right_of.get_mut(&i);
            if r.is_none() {
                right_of.insert(*i, vec![*k]);
            } else {
                r.unwrap().push(*k);
            }
        }
    }

    right_of
}

fn custom_sort_by(
    x: i32,
    y: i32,
    left_of: &HashMap<i32, Vec<i32>>,
    right_of: &HashMap<i32, Vec<i32>>,
) -> core::cmp::Ordering {
    let _r = right_of.get(&x);
    let _l = left_of.get(&x);

    if _l.is_some() && _l.unwrap().contains(&y) {
        core::cmp::Ordering::Less
    } else if _r.is_some() && _r.unwrap().contains(&y) {
        core::cmp::Ordering::Greater
    } else {
        core::cmp::Ordering::Equal
    }
}

fn part_a(lines: &Vec<String>) -> i64 {
    let (left_of, right_of, pages) = split_input(lines);

    let mut result = 0;
    for p in pages {
        let mut ordered_page = p.clone();
        ordered_page.sort_by(|x, y| custom_sort_by(*x, *y, &left_of, &right_of));
        if ordered_page == p {
            let i = ordered_page.len() / 2;
            let middle = ordered_page[i];
            result += middle as i64;
        }
    }

    result
}

fn part_b(lines: &Vec<String>) -> i64 {
    let (left_of, right_of, pages) = split_input(lines);

    let mut result = 0;
    for p in pages {
        let mut ordered_page = p.clone();
        ordered_page.sort_by(|x, y| custom_sort_by(*x, *y, &left_of, &right_of));
        if ordered_page != p {
            let i = ordered_page.len() / 2;
            let middle = ordered_page[i];
            result += middle as i64;
        }
    }

    result
}
