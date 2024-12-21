extern crate num;
use crate::commons::read_file::read_to_vec_str;
use num::complex::Complex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub(crate) fn run() {
    println!("part B");
    let lines = read_to_vec_str("test.txt").unwrap();
    println!("Test result: {}", part_b(&lines));

    let lines = read_to_vec_str("input.txt").unwrap();
    let before = Instant::now();
    println!("Input result: {}", part_b(&lines));
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn to_grid(lines: &Vec<String>) -> HashMap<Complex<i32>, char> {
    let m = lines.len();
    let n = lines[0].len();
    let mut grid: HashMap<Complex<i32>, char> = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            let c = Complex::new(i as i32, j as i32);
            grid.insert(c, lines[i].chars().nth(j).unwrap());
        }
    }

    grid
}

fn get_starting_position(grid: &HashMap<Complex<i32>, char>) -> Complex<i32> {
    let mut starting_position = Complex::new(0, 0);
    for (k, v) in grid {
        if *v == '^' {
            starting_position = *k;
            break;
        }
    }

    starting_position
}

#[derive(Eq, Hash, PartialEq)]
struct Visited {
    position: Complex<i32>,
    update: Complex<i32>,
}

fn part_b(lines: &Vec<String>) -> i32 {
    let mut grid = to_grid(lines);
    let starting_position = get_starting_position(&grid);
    let max_row = lines.len();
    let max_col = lines[0].len();

    // replace starting position char
    grid.insert(starting_position, '|');

    let mut counter = 0;
    let mut n_loops = 0;
    for i in 0..max_row {
        for j in 0..max_col {
            counter += 1;
            if counter % 1000 == 0 {
                println!("Iter {counter} / {}", max_col * max_row);
            }
            let obstacle_position = Complex::new(i as i32, j as i32);
            let prev_obstacle = grid.get(&obstacle_position).unwrap().clone();
            if obstacle_position != starting_position {
                // update grid
                grid.insert(obstacle_position, '#');
                // current update
                let mut update = Complex::new(-1, 0);
                // track tuples of visited positions and orientations at that position (position, orientation)
                let mut visited: HashSet<Visited> = HashSet::new();
                let v = Visited {
                    position: starting_position,
                    update: update,
                };
                visited.insert(v);
                let mut position = starting_position;
                loop {
                    let new_position = position + update;
                    if grid.contains_key(&new_position) {
                        let g = grid.get(&new_position).unwrap();
                        let newly_visited = Visited {
                            position: new_position,
                            update: update,
                        };
                        if visited.contains(&newly_visited) {
                            n_loops += 1;
                            break;
                        } else if *g == '#' {
                            update *= Complex::new(0, -1);
                            visited.insert(newly_visited);
                        } else {
                            position = new_position;
                            visited.insert(newly_visited);
                        }
                    } else {
                        // guard leaves the grid
                        break;
                    }
                }

                // revert grid for new iteration
                grid.insert(obstacle_position, prev_obstacle);
            }
        }
    }

    n_loops
}
