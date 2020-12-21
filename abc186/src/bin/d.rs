use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    }

    a.sort();
    let a = a;
    let mut v: VecDeque<i64> = VecDeque::new();

    let mut tmp = a.clone();
    tmp.reverse();
    let mut acc: i64 = 0;
    for i in tmp.into_iter().take(n as usize - 1) {
        acc += i;
        v.push_front(acc);
    }

    let mut ans: i64 = 0;
    for i in 0..n - 1 {
        ans += v[i as usize] - a[i as usize] * (n - 1 - i);
    }
    println!("{}", ans)
}
