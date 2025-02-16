extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use num::Complex;
use std::collections::{HashMap, VecDeque};

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(36));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(667));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(81));
    show_and_check("B", false, part_b(&input_lines) as i64, Some(1344));
}

fn make_grid(lines: &Vec<String>) -> HashMap<Complex<i32>, i32> {
    let mut grid = HashMap::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            grid.insert(
                Complex::new(x as i32, y as i32),
                c.to_digit(10).unwrap() as i32,
            );
        }
    }
    grid
}

fn trailhead_positions(grid: &HashMap<Complex<i32>, i32>) -> Vec<Complex<i32>> {
    let mut positions = vec![];
    for (pos, c) in grid.iter() {
        if *c == 0 {
            positions.push(*pos);
        }
    }
    positions
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Trail {
    head: Complex<i32>,
    pos: Complex<i32>,
    visited: Vec<Complex<i32>>,
}

fn solve(lines: &Vec<String>, is_part_b: bool) -> i64 {
    let updates = vec![
        Complex::new(0, 1),
        Complex::new(0, -1),
        Complex::new(1, 0),
        Complex::new(-1, 0),
    ];
    let grid = make_grid(lines);
    let mut open_trails: VecDeque<Trail> = trailhead_positions(&grid)
        .iter()
        .map(|p| Trail {
            head: *p,
            pos: *p,
            visited: vec![*p],
        })
        .collect();
    let mut closed_trails: Vec<Trail> = vec![];

    while !open_trails.is_empty() {
        let t = open_trails.pop_front().unwrap();
        if *grid.get(&t.pos).unwrap() == 9 {
            // reached top
            closed_trails.push(t);
            continue;
        }

        // need additional step
        for update in updates.iter() {
            let new_pos = t.pos + update;
            if grid.contains_key(&new_pos) {
                if t.visited.contains(&new_pos) {
                    continue;
                }
                let current_c = *grid.get(&t.pos).unwrap();
                let new_c = *grid.get(&new_pos).unwrap();
                if (new_c - current_c) != 1 {
                    continue;
                }
                let mut new_visited = t.visited.clone();
                new_visited.push(new_pos);
                open_trails.push_back(Trail {
                    head: t.head,
                    pos: new_pos,
                    visited: new_visited,
                });
            }
        }
    }

    if is_part_b {
        closed_trails.len() as i64
    } else {
        let mut reduced_trails: HashMap<(Complex<i32>, Complex<i32>), Trail> = HashMap::new();
        for t in closed_trails {
            let key = (t.head, *t.visited.last().unwrap());
            reduced_trails.insert(key, t);
        }
        reduced_trails.len() as i64
    }
}

fn part_a(lines: &Vec<String>) -> i64 {
    solve(lines, false)
}

fn part_b(lines: &Vec<String>) -> i64 {
    solve(lines, true)
}
