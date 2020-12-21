use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        s: String,
        q: i32,
    }

    let mut v: Vec<(i32, char)> = vec![];
    let mut rev_cnt = 0;
    for _ in 0..q {
        input! {
            t: u8
        }
        if t == 1 {
            rev_cnt += 1;
        } else {
            input! {
                f: u8,
                c: char
            }
            let add = match (rev_cnt % 2, f) {
                (0, 1) => 1,
                (0, 2) => 2,
                (1, 1) => 2,
                (1, 2) => 1,
                _ => 0,
            };
            v.push((add, c));
        }
    }
    let rev_cnt = rev_cnt;
    let mut s: VecDeque<char> = s.chars().collect();
    for (k, v) in v {
        if k == 1 {
            s.push_front(v);
        } else {
            s.push_back(v);
        }
    }
    if rev_cnt % 2 == 0 {
        for i in s {
            print!("{}", i);
        }
    } else {
        for i in s.into_iter().rev() {
            print!("{}", i);
        }
    }
    println!();
}
