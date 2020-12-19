use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: String,
        k: usize
    }

    let mut ans = 0;
    let mut count = 1;
    let mut prev = s.clone().chars().nth(0).unwrap();
    let head = prev;
    let a = s.clone().chars().take_while(|&ch| ch == head).count();
    let b = s.clone().chars().rev().take_while(|&ch| ch == head).count();
    for next in s.clone().chars().skip(1) {
        if prev == next {
            count += 1;
        } else {
            ans += count / 2;
            count = 1;
            prev = next;
        }
    }
    ans += count / 2;
    let last = prev;

    println!(
        "{}",
        if s.clone().chars().all(|ch| ch == head) {
            (s.len() * k) / 2
        } else if head == last {
            ans * k - (a / 2 + b / 2 - (a + b) / 2) * (k - 1)
        } else {
            ans * k
        }
    );
}
