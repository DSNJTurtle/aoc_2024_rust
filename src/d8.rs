extern crate itertools;
extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use itertools::Itertools;
use num::Complex;
use std::collections::HashMap;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(14));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(396));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(34));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(1200));
}

fn parse_grid(lines: &Vec<String>) -> HashMap<Complex<i32>, char> {
    let mut grid = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert(Complex::new(j as i32, i as i32), c);
        }
    }
    grid
}

fn distinct_frequencies(grid: &HashMap<Complex<i32>, char>) -> HashMap<char, Vec<Complex<i32>>> {
    grid.iter()
        .into_group_map_by(|x| *x.1)
        .into_iter()
        .filter(|(k, _)| *k != '.')
        .map(|(k, v)| {
            (
                k,
                v.into_iter().map(|x| *x.0).collect::<Vec<Complex<i32>>>(),
            )
        })
        .collect::<HashMap<char, Vec<Complex<i32>>>>()
}

fn part_a(lines: &Vec<String>) -> i64 {
    let grid = parse_grid(lines);
    let m = lines.len() as i32;
    let n = lines[0].len() as i32;

    let frequencies = distinct_frequencies(&grid);
    let mut antinodes: HashMap<char, Vec<Complex<i32>>> = HashMap::new();
    for (k, v) in frequencies.iter() {
        // pairwise diff
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let d = v[i] - v[j];
                let antinode = v[i] + d;
                if antinode.re >= 0 && antinode.re < m && antinode.im >= 0 && antinode.im < n {
                    let x = antinodes.get_mut(k);
                    if x.is_none() {
                        antinodes.insert(*k, vec![antinode]);
                    } else {
                        x.unwrap().push(antinode);
                    }
                }
            }
        }
    }

    let result = antinodes.into_iter().flat_map(|(_, v)| v).unique().count() as i64;

    result
}

fn part_b(lines: &Vec<String>) -> i64 {
    // Note: This solution is not cleaned up and can be optimised.
    let grid = parse_grid(lines);
    let m = lines.len() as i32;
    let n = lines[0].len() as i32;

    let frequencies = distinct_frequencies(&grid);

    let mut antinodes: HashMap<char, Vec<Complex<i32>>> = HashMap::new();
    for (k, v) in frequencies.iter() {
        // pairwise diff
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                if i == j {
                    continue;
                }
                let mut d = v[i] - v[j];
                // make sure we have the smallest step size
                let gcd = num::integer::gcd(d.re.abs(), d.im.abs());
                d = d / gcd;
                // go in forward and backward direction
                let mut antinode = v[i] + d;
                let mut antinode_neg = v[i] - d;
                while antinode.re >= 0 && antinode.re < m && antinode.im >= 0 && antinode.im < n {
                    let x = antinodes.get_mut(k);
                    if x.is_none() {
                        antinodes.insert(*k, vec![antinode, v[i], v[j]]);
                    } else {
                        x.unwrap().extend(vec![antinode, v[i], v[j]]);
                    }

                    antinode += d;
                }
                while antinode_neg.re >= 0
                    && antinode_neg.re < m
                    && antinode_neg.im >= 0
                    && antinode_neg.im < n
                {
                    let x = antinodes.get_mut(k);
                    if x.is_none() {
                        antinodes.insert(*k, vec![antinode_neg, v[i], v[j]]);
                    } else {
                        x.unwrap().extend(vec![antinode_neg, v[i], v[j]]);
                    }

                    antinode_neg -= d;
                }
            }
        }
    }

    let result = antinodes.into_iter().flat_map(|(_, v)| v).unique().count() as i64;

    result
}
