use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        mut s: String
    }

    let mut prev = s.pop().unwrap();
    let mut cnt = 1;
    let mut v: VecDeque<(char, i32)> = VecDeque::new();
    while !s.is_empty() {
        let next = s.pop().unwrap();
        if prev == next {
            cnt += 1;
        } else {
            v.push_front((prev, cnt));
            cnt = 1;
            prev = next;
        }
    }
    v.push_front((prev, cnt));

    for (ch, i) in v {
        if ch != ' ' {
            print!("{}{}", ch, i);
        } else {
            print!(" ");
        }
    }
    println!();
}
