use std::any::Any;
use std::rc::Rc;
use std::{collections::HashSet, io::stdin};

/* HEAP STACK CODE */

enum HeapStackReturn {
    Call(Box<dyn HeapStackPlate>),
    Return(Box<dyn Any>),
}

trait HeapStackPlate {
    fn call(
        self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>);
}

fn run_stack(plate: Box<dyn HeapStackPlate>) -> Box<dyn Any> {
    let mut stack: Vec<Box<dyn HeapStackPlate>> = vec![plate];

    let mut last_return: Option<Box<dyn Any>> = None;

    while let Some(top) = stack.pop() {
        match top.call(last_return.take()) {
            (HeapStackReturn::Call(new_plate), new_self) => {
                stack.push(new_self);
                stack.push(new_plate);

                last_return = None;
            }

            (HeapStackReturn::Return(val), _) => {
                last_return = Some(val);
            }
        }
    }

    last_return.take().unwrap()
}

/* HEAP STACK CODE */

#[derive(Copy, Clone, Debug)]
enum SolutionState {
    Start,
    AfterFirstBranch,
    Done,
}

struct SolutionPlate {
    path: Rc<[char]>,
    position: usize,
    level: i64,
    current_max: i64,
    max_position_set: HashSet<usize>,

    state: SolutionState,
}

impl HeapStackPlate for SolutionPlate {
    fn call(
        mut self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>) {
        match self.state {
            SolutionState::Start => {
                match special_max(self.level, self.current_max) {
                    MaxResult::Same => {
                        self.max_position_set.insert(self.position);
                    }
                    MaxResult::LeftGrater => {
                        self.max_position_set.clear();
                        self.max_position_set.insert(self.position);
                        self.current_max = self.level;
                    }
                    MaxResult::RightGrater => {} // do nothing
                }

                if self.position >= self.path.len() {
                    let max_position_set = self.max_position_set;
                    self.max_position_set = HashSet::new();

                    return (HeapStackReturn::Return(Box::new(max_position_set)), self);
                }

                match self.path[self.position] {
                    '+' => {
                        self.state = SolutionState::Done;

                        let max_position_set = self.max_position_set;
                        self.max_position_set = HashSet::new();

                        (
                            HeapStackReturn::Call(Box::new(SolutionPlate {
                                path: Rc::clone(&self.path),
                                position: self.position + 1,
                                level: self.level + 1,
                                current_max: self.current_max,
                                max_position_set,
                                state: SolutionState::Start,
                            })),
                            self,
                        )
                    }
                    '-' => {
                        self.state = SolutionState::Done;

                        let max_position_set = self.max_position_set;
                        self.max_position_set = HashSet::new();

                        (
                            HeapStackReturn::Call(Box::new(SolutionPlate {
                                path: Rc::clone(&self.path),
                                position: self.position + 1,
                                level: self.level - 1,
                                current_max: self.current_max,
                                max_position_set,
                                state: SolutionState::Start,
                            })),
                            self,
                        )
                    }
                    '?' => {
                        self.state = SolutionState::AfterFirstBranch;

                        let max_position_set = self.max_position_set.clone();

                        (
                            HeapStackReturn::Call(Box::new(SolutionPlate {
                                path: Rc::clone(&self.path),
                                position: self.position + 1,
                                level: self.level + 1,
                                current_max: self.current_max,
                                max_position_set,
                                state: SolutionState::Start,
                            })),
                            self,
                        )
                    }
                    _ => panic!("Invalid character in path"),
                }
            }
            SolutionState::AfterFirstBranch => {
                self.state = SolutionState::Done;

                let max_position_set = self.max_position_set;
                self.max_position_set = *last_return
                    .unwrap()
                    .downcast::<HashSet<usize>>()
                    .unwrap();

                (
                    HeapStackReturn::Call(Box::new(SolutionPlate {
                        path: Rc::clone(&self.path),
                        position: self.position + 1,
                        level: self.level - 1,
                        current_max: self.current_max,
                        max_position_set,
                        state: SolutionState::Start,
                    })),
                    self,
                )
            }
            SolutionState::Done => {
                let temp_set = *last_return
                        .unwrap()
                        .downcast::<HashSet<usize>>()
                        .unwrap();
                
                let result_set: HashSet<usize> = self.max_position_set
                    .union(&temp_set)
                    .cloned() 
                    .collect();

                (HeapStackReturn::Return(Box::new(result_set)), self)
            }
        }
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

fn main() {
    let _ = stdin()
        .read_line(&mut String::new())
        .expect("Failed to read the first parameter");

    let mut path: String = String::new();
    let _ = stdin()
        .read_line(&mut path)
        .expect("Failed to read the path(second parameter)");

    let path: Rc<[char]> = path.trim().chars().collect::<Vec<char>>().into();

    let starting_level: i64 = 0;
    let starting_position: usize = 0;

    let max_position_set = run_stack(Box::new(SolutionPlate {
        path: Rc::clone(&path),
        position: starting_position,
        level: starting_level,
        current_max: starting_level,
        max_position_set: HashSet::new(),

        state: SolutionState::Start,
    }));

    // let mut max_position_set = solve(
    //     &path,
    //     starting_position,
    //     starting_level,
    //     starting_level,
    //     HashSet::new(),
    // )
    // .into_iter()
    // .collect::<Vec<usize>>();

    // max_position_set.sort_unstable();

    let mut max_position_set = max_position_set
        .downcast::<HashSet<usize>>()
        .unwrap()
        .into_iter()
        .collect::<Vec<usize>>();

    max_position_set.sort_unstable();

    println!("{}", max_position_set.len());
    for p in max_position_set {
        print!("{} ", p);
    }
    println!();
}
