use std::{collections::HashSet, io::stdin};
use std::any::Any

/* HEAP STACK CODE */

enum HeapStackReturn {
    Call(Box<dyn HeapStackPlate>),
    Return(Box<dyn Any>),
}

trait HeapStackPlate {
    fn call(&mut self, last_return: Option<Box<dyn Any>>) -> HeapStackReturn;
}

fn run_stack(plate: Box<dyn HeapStackPlate>) -> Box<dyn Any> {
    let mut stack: Vec<Box<dyn HeapStackPlate>> = vec![plate];

    let mut last_return: Option<Box<dyn Any>> = None;

    while let Some(top) = stack.last_mut() {
        match top.call(last_return.take()) {
            HeapStackReturn::Call(new_plate) => {
                stack.push(new_plate);

                last_return = None;
            }
            HeapStackReturn::Return(val) => {
                stack.pop();

                last_return = Some(val);
            }
        }
    }

    last_return.take().unwrap()
}

/* HEAP STACK CODE */

struct SolutionPlate<'a> {
    path: &'a Box<[char]>,
    position: usize,
    level: i64,
    current_max: i64,
    max_position_set: HashSet<usize>,
}

impl<'a> HeapStackPlate for SolutionPlate<'a> {
    fn call(&mut self, last_return: Option<Box<dyn Any>>) -> HeapStackReturn {

    }
}

enum MaxResult {
    Same,
    LeftGrater,
    RightGrater,
}

fn special_max(l: i64, r: i64) -> MaxResult {
    if l == r {
        MaxResult::Same
    } else if l > r {
        MaxResult::LeftGrater
    } else {
        MaxResult::RightGrater
    }
}

fn solve(
    path: &Box<[char]>,
    position: usize,
    level: i64,
    mut current_max: i64,
    mut max_position_set: HashSet<usize>,
) -> HashSet<usize> {
    match special_max(level, current_max) {
        MaxResult::Same => {
            max_position_set.insert(position);
        }
        MaxResult::LeftGrater => {
            max_position_set.clear();
            max_position_set.insert(position);
            current_max = level
        }
        MaxResult::RightGrater => {} // do nothing
    }

    if position >= path.len() {
        return max_position_set;
    }

    match path[position] {
        '+' => {
            max_position_set = solve(path, position + 1, level + 1, current_max, max_position_set);
        }
        '-' => {
            max_position_set = solve(path, position + 1, level - 1, current_max, max_position_set);
        }
        '?' => {
            let max_index_set1 = solve(
                path,
                position + 1,
                level + 1,
                current_max,
                max_position_set.clone(),
            );
            let max_index_set2 =
                solve(path, position + 1, level - 1, current_max, max_position_set);

            max_position_set = max_index_set1.union(&max_index_set2).copied().collect();
        }
        _ => panic!("Invalid character in path"),
    }

    max_position_set
}

fn main() {
    let _ = stdin()
        .read_line(&mut String::new())
        .expect("Failed to read the first parameter");

    let mut path: String = String::new();
    let _ = stdin()
        .read_line(&mut path)
        .expect("Failed to read the path(second parameter)");

    let path: Box<[char]> = path.trim().chars().collect::<Vec<char>>().into();

    let starting_level: i64 = 0;
    let starting_position: usize = 0;

    let mut max_position_set: HashSet<usize> = HashSet::new();
    max_position_set.insert(starting_position);

    match path.first() {
        Some('+') => {
            let mut max_position_set = solve(
                &path,
                starting_position + 1,
                starting_level + 1,
                0,
                max_position_set,
            )
            .into_iter()
            .collect::<Vec<usize>>();

            max_position_set.sort_unstable();

            println!("{}", max_position_set.len());
            for p in max_position_set {
                print!("{} ", p);
            }
            println!()
        }
        Some('-') => {
            let mut max_position_set = solve(
                &path,
                starting_position + 1,
                starting_level - 1,
                0,
                max_position_set,
            )
            .into_iter()
            .collect::<Vec<usize>>();

            max_position_set.sort_unstable();

            println!("{}", max_position_set.len());
            for (i, p) in max_position_set.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{}", p);
            }
            println!();
        }
        Some('?') => {
            let max_position_set1 = solve(
                &path,
                starting_position + 1,
                starting_level + 1,
                0,
                max_position_set.clone(),
            );
            let max_position_set2 = solve(
                &path,
                starting_position + 1,
                starting_level - 1,
                0,
                max_position_set,
            );

            let mut max_position_set = max_position_set1
                .union(&max_position_set2)
                .copied()
                .collect::<Vec<usize>>();

            max_position_set.sort_unstable();

            println!("{}", max_position_set.len());
            for p in max_position_set {
                print!("{} ", p);
            }
            println!();
        }
        _ => panic!("Invalid first character in path"),
    }
}
