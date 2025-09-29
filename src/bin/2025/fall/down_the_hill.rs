use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::stdin,
    rc::Rc,
};

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

thread_local! {
    static SOLUTION_CACHE: RefCell<HashMap<(usize, i64), HashSet<usize>>> = RefCell::new(HashMap::new());
}

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

impl SolutionPlate {
    fn cache_set_strict(&mut self, set_to_cache: &HashSet<usize>) {
        if self.position == 0 {
            return;
        }

        if let Some('?') = self.path.get(self.position - 1) {
            SOLUTION_CACHE.with_borrow_mut(|cache| {
                let _ = cache.insert((self.position, self.level), set_to_cache.clone());
            });
        }
    }

    fn get_cached(position: usize, level: i64) -> Option<HashSet<usize>> {
        SOLUTION_CACHE.with_borrow(|cache| cache.get(&(position, level)).cloned())
    }

    fn start(
        mut self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>) {
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

            self.cache_set_strict(&max_position_set);

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

                if let Some(cached) = Self::get_cached(self.position + 1, self.level + 1) {
                    return self.after_first_branch(Some(Box::new(cached)));
                }

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

    fn after_first_branch(
        mut self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>) {
        self.state = SolutionState::Done;

        let last_return_set = *last_return.unwrap().downcast::<HashSet<usize>>().unwrap();

        let max_position_set = self.max_position_set;
        self.max_position_set = last_return_set;

        if let Some(cached) = Self::get_cached(self.position + 1, self.level - 1) {
            return self.done(Some(Box::new(cached)));
        }

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

    fn done(
        mut self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>) {
        let temp_set = *last_return.unwrap().downcast::<HashSet<usize>>().unwrap();

        let result_set = if self.max_position_set.len() > 0 {
            self.max_position_set
                .union(&temp_set)
                .cloned()
                .collect::<HashSet<usize>>()
        } else {
            temp_set
        };

        self.cache_set_strict(&result_set);

        (HeapStackReturn::Return(Box::new(result_set)), self)
    }
}

impl HeapStackPlate for SolutionPlate {
    fn call(
        self: Box<Self>,
        last_return: Option<Box<dyn Any>>,
    ) -> (HeapStackReturn, Box<dyn HeapStackPlate>) {
        match self.state {
            SolutionState::Start => self.start(last_return),
            SolutionState::AfterFirstBranch => self.after_first_branch(last_return),
            SolutionState::Done => self.done(last_return),
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
