extern crate num;
use crate::commons::read_file::read_to_vec_str;
use crate::commons::show_and_check::show_and_check;
use num::complex::Complex;
use std::collections::HashSet;
use std::time::Instant;

pub(crate) fn run() {
    let test_lines = read_to_vec_str("test.txt").unwrap();
    let input_lines = read_to_vec_str("input.txt").unwrap();

    show_and_check("A", true, part_a(&test_lines) as i64, Some(41));
    show_and_check("A", false, part_a(&input_lines) as i64, Some(5564));

    show_and_check("B", true, part_b(&test_lines) as i64, Some(6));
    let before = Instant::now();
    show_and_check("B", false, part_b(&input_lines) as i64, Some(1976));
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn to_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    let m = lines.len();
    let n = lines[0].len();
    let mut grid = vec![vec!['.'; n]; m];
    for i in 0..m {
        for j in 0..n {
            grid[i][j] = lines[i].chars().nth(j).unwrap();
        }
    }

    grid
}

fn get_starting_position(grid: &Vec<Vec<char>>) -> Complex<i32> {
    let mut starting_position = Complex::new(0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                starting_position = Complex::new(i as i32, j as i32);
                break;
            }
        }
    }

    starting_position
}

#[inline(always)]
fn update_position(position: Complex<i32>, update: Complex<i32>) -> Complex<i32> {
    position + update
}

#[inline]
fn update_orientation(update: Complex<i32>) -> Complex<i32> {
    update * Complex::new(0, -1)
}

#[derive(Eq, Hash, PartialEq)]
struct Visited {
    position: Complex<i32>,
    update: Complex<i32>,
}

fn get_visited_positions(
    grid: &Vec<Vec<char>>,
    start_position: Complex<i32>,
    start_orientation: Complex<i32>,
) -> HashSet<Complex<i32>> {
    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut update = start_orientation;

    let mut position = start_position;
    let mut visited: HashSet<Complex<i32>> = HashSet::new();
    visited.insert(start_position);

    loop {
        let new_position = update_position(position, update);
        if new_position.re < max_row as i32
            && new_position.re >= 0
            && new_position.im < max_col as i32
            && new_position.im >= 0
        {
            if grid[new_position.re as usize][new_position.im as usize] == '#' {
                update = update_orientation(update);
            } else {
                visited.insert(new_position);
                position = new_position;
            }
        } else {
            break;
        }
    }

    visited
}

fn part_a(lines: &Vec<String>) -> i32 {
    let grid = to_grid(lines);
    let starting_position = get_starting_position(&grid);

    let update = Complex::new(-1, 0);
    let visited = get_visited_positions(&grid, starting_position, update);

    visited.len() as i32
}

fn part_b(lines: &Vec<String>) -> i32 {
    let mut grid = to_grid(lines);
    let starting_position = get_starting_position(&grid);
    let max_row = lines.len();
    let max_col = lines[0].len();

    // replace starting position char
    grid[starting_position.re as usize][starting_position.im as usize] = '|';

    let mut n_loops = 0;
    let relevant_positions = get_visited_positions(&grid, starting_position, Complex::new(-1, 0));

    for pos in relevant_positions {
        let i = pos.re as usize;
        let j = pos.im as usize;

        let obstacle_position = Complex::new(i as i32, j as i32);
        let prev_obstacle = grid[i][j];
        if obstacle_position != starting_position {
            // update grid
            grid[obstacle_position.re as usize][obstacle_position.im as usize] = '#';
            // current update
            let mut update = Complex::new(-1, 0);
            // track tuples of visited positions and orientations at that position (position, orientation)
            let mut visited: HashSet<Visited> = HashSet::new();
            let v = Visited {
                position: starting_position,
                update,
            };
            visited.insert(v);

            let mut position = starting_position;
            loop {
                let new_position = update_position(position, update);
                if new_position.re < max_row as i32
                    && new_position.re >= 0
                    && new_position.im < max_col as i32
                    && new_position.im >= 0
                {
                    if grid[new_position.re as usize][new_position.im as usize] == '#' {
                        update = update_orientation(update);
                    } else {
                        position = new_position;
                    }
                    let newly_visited = Visited {
                        position: new_position,
                        update,
                    };
                    if visited.contains(&newly_visited) {
                        n_loops += 1;
                        break;
                    } else {
                        visited.insert(newly_visited);
                    }
                } else {
                    // guard leaves the grid
                    break;
                }
            }

            // revert grid for new iteration
            grid[obstacle_position.re as usize][obstacle_position.im as usize] = prev_obstacle;
        }
    }

    n_loops
}
