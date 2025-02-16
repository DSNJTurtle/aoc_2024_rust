extern crate num;

use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use num::Complex;
use std::collections::HashMap;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines), Some(1930));
    show_and_check("A", false, part_a(&input_lines), Some(1488414));

    show_and_check("B", true, part_b(&test_lines), Some(1206));
    show_and_check("B", false, part_b(&input_lines), Some(911750));
}

fn make_grid(lines: &Vec<String>) -> HashMap<Complex<i32>, char> {
    let mut grid = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(Complex::new(x as i32, y as i32), c);
        }
    }
    grid
}

fn find_connected_area(grid: &HashMap<Complex<i32>, char>, pos: Complex<i32>) -> Vec<Complex<i32>> {
    let plant = grid[&pos];
    let mut connected = vec![];
    let mut to_check = vec![pos];

    while !to_check.is_empty() {
        let current = to_check.pop().unwrap();
        if connected.contains(&current) {
            continue;
        }
        connected.push(current);
        let neighbours = vec![
            current + Complex::new(0, 1),
            current + Complex::new(0, -1),
            current + Complex::new(1, 0),
            current + Complex::new(-1, 0),
        ];
        for n in neighbours {
            if grid.contains_key(&n) && grid[&n] == plant {
                to_check.push(n);
            }
        }
    }
    connected
}

fn find_all_connected_areas(grid: &HashMap<Complex<i32>, char>) -> Vec<Vec<Complex<i32>>> {
    let mut areas = vec![];
    let mut grid = grid.clone();
    while !grid.is_empty() {
        let pos = *grid.keys().next().unwrap();
        let connected = find_connected_area(&grid, pos);
        for c in &connected {
            grid.remove(c);
        }
        areas.push(connected);
    }
    areas
}

fn compute_perimeter_of_area(grid: &HashMap<Complex<i32>, char>, area: &Vec<Complex<i32>>) -> u64 {
    let mut perimeter = 0;
    for pos in area {
        let neighbours = vec![
            pos + Complex::new(0, 1),
            pos + Complex::new(0, -1),
            pos + Complex::new(1, 0),
            pos + Complex::new(-1, 0),
        ];
        for n in neighbours {
            if !area.contains(&n) || !grid.contains_key(&n) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn compute_sides_of_area(area: &Vec<Complex<i32>>) -> u64 {
    // hint from subreddit: number of corners = number of sides
    let mut sides = 0;
    let diagonals = vec![
        Complex::new(1, 1),
        Complex::new(1, -1),
        Complex::new(-1, 1),
        Complex::new(-1, -1),
    ];

    for pos in area {
        for d in &diagonals {
            let row_ngh = pos + Complex::new(d.re, 0);
            let col_ngh = pos + Complex::new(0, d.im);
            let diag_ngh = pos + d;

            // external corners
            if !area.contains(&row_ngh) && !area.contains(&col_ngh) {
                sides += 1;
            }

            // internal corners
            if area.contains(&row_ngh) && area.contains(&col_ngh) && !area.contains(&diag_ngh) {
                sides += 1;
            }
        }
    }

    sides
}

fn part_a(lines: &Vec<String>) -> u64 {
    let grid = make_grid(lines);
    let areas = find_all_connected_areas(&grid);
    let perimeters: Vec<u64> = areas
        .iter()
        .map(|a| compute_perimeter_of_area(&grid, a))
        .collect();

    let mut sum = 0;
    for i in 0..areas.len() {
        sum += areas[i].len() as u64 * perimeters[i];
    }
    sum
}

fn part_b(lines: &Vec<String>) -> u64 {
    let grid = make_grid(lines);
    let areas = find_all_connected_areas(&grid);
    let sides: Vec<u64> = areas.iter().map(|a| compute_sides_of_area(a)).collect();

    let mut sum = 0;
    for i in 0..areas.len() {
        sum += areas[i].len() as u64 * sides[i];
    }
    sum
}
