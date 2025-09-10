/// # Exercise: Please Wait
///
/// Imagine a typical day at the company "Coding With Breaks." At 10:50 AM, all hungry programmers
/// ask the same question: "When will lunch be ready?"
///
/// In this company, lunch doesn't arrive in a chaotic manner. On a large screen in the kitchen,
/// a long sequence of zeros and ones indicates the current status:
/// - Block of zeros: "Lunch is still being prepared."
/// - Block of ones: "Lunch is ready, you can go eat."
///
/// Naturally, the signal must always appear in order — first all zeros, then all ones.
/// However, as is often the case, the system occasionally malfunctions. Instead of a nice transition
/// from zeros to ones, the screen displays something like this:
///
/// ```
/// 0101110010
/// ```
///
/// The result is panic in the hallway. Should we go eat? Should we wait? Should we order pizza?
///
/// The programmers, however, want to help. They assume that the signal has the smallest possible
/// number of errors, and they try to determine how many numbers need to be changed so that the signal
/// is corrected and everyone finally knows what to do.
///
/// ## Task
/// You are given a sequence consisting of zeros and ones. Determine the minimum number of changes
/// needed to transform the given sequence into the desired form: `000...111`. To achieve this,
/// the sequence must consist of a continuous block of zeros followed by a continuous block of ones,
/// where each block must have a length of at least 1.
///
/// ## Input Format
/// The first line of input contains an integer `n` (2 ≤ n ≤ 10^6) representing the length of the sequence.
/// The second line contains the sequence consisting of `n` characters, each being either `0` or `1`.
///
/// ## Output Format
/// Print a single integer representing the minimum number of changes needed to transform the sequence
/// into the desired state.
///
/// ## Example
/// ### Input:
/// ```
/// 7
/// 0101101
/// ```
///
/// ### Output:
/// ```
/// 2
/// ```
///
/// ### Explanation:
/// We can change the zeros at the third and sixth positions to ones, resulting in the sequence `0111111`.
/// Alternatively, we can change the one at the second position and the zero at the sixth position,
/// resulting in the sequence `0001111`.
use std::{io::stdin, rc::Rc};

fn main() {
    let mut lunch_schedual = String::new();

    let _ = stdin().read_line(&mut String::new()).unwrap();
    let _ = stdin().read_line(&mut lunch_schedual).unwrap();

    let lunch_schedual: Rc<str> = lunch_schedual.trim().into();
    let computing_slice: &str = &lunch_schedual.chars().as_str()[1..lunch_schedual.len() - 1];

    let mut number_of_zeros_right_of_pointer: usize = 0;
    computing_slice.chars().for_each(|e| {
        if e == '0' {
            number_of_zeros_right_of_pointer += 1
        }
    });

    let mut number_of_ones_left_of_pointer: usize = 0;

    let mut best: usize = number_of_zeros_right_of_pointer;

    for ch in computing_slice.chars() {
        match ch {
            '0' => {
                number_of_zeros_right_of_pointer -= 1;
            }
            '1' => {
                number_of_ones_left_of_pointer += 1;
            }
            _ => panic!("invalid input! {}", ch),
        }
        best = best.min(number_of_ones_left_of_pointer + number_of_zeros_right_of_pointer);
    }

    if let Some('1') = lunch_schedual.chars().nth(0) {
        best += 1;
    }

    if let Some('0') = lunch_schedual.chars().nth_back(0) {
        best += 1;
    }

    println!("{}", best);
}
