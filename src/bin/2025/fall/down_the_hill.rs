use std::io::{self, Read};

fn main() {
    // read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let s: Vec<char> = it.next().unwrap().chars().collect();

    // 1) prefix S with v(+)=+1, v(-)=-1, v(?)=-1
    let mut s_val = vec![0i32; n + 1];
    for i in 1..=n {
        let v = match s[i - 1] {
            '+' => 1,
            '-' => -1,
            '?' => -1,
            _ => unreachable!(),
        };
        s_val[i] = s_val[i - 1] + v;
    }
    // need[i] = max_{t<=i} S[t]
    let mut need = vec![0i32; n + 1];
    let mut cur_max = 0i32;
    for i in 1..=n {
        cur_max = cur_max.max(s_val[i]);
        need[i] = cur_max;
    }

    // 2) bal and q, then maxH
    let mut bal = vec![0i32; n + 1];
    let mut q = vec![0i32; n + 1];
    for i in 1..=n {
        bal[i] = bal[i - 1]
            + match s[i - 1] {
                '+' => 1,
                '-' => -1,
                _ => 0,
            };
        q[i] = q[i - 1] + if s[i - 1] == '?' { 1 } else { 0 };
    }
    let max_h = |i: usize| bal[i] + q[i];

    // 3) maxSuffixS
    let mut max_suf = vec![i32::MIN; n + 2];
    max_suf[n] = s_val[n];
    for i in (0..n).rev() {
        max_suf[i] = max_suf[i + 1].max(s_val[i]);
    }

    // 4) collect answers
    let mut ans = Vec::new();
    for i in 0..=n {
        let ok_prefix = need[i] <= max_h(i);
        let ok_suffix = if i == n {
            true
        } else {
            max_suf[i + 1] <= s_val[i]
        };
        let ok_peak = i == 0 || s[i - 1] != '-';
        if ok_prefix && ok_suffix && ok_peak {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    for (k, x) in ans.iter().enumerate() {
        if k > 0 {
            print!(" ");
        }
        print!("{}", x);
    }
    println!();
}
