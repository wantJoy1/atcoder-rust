use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        ab: [(i32, i32); m]
    }

    let mut hs_1: HashSet<i32> = HashSet::new();
    let mut hs_n: HashSet<i32> = HashSet::new();
    for (a, b) in ab {
        if a == 1 {
            hs_1.insert(b);
        }
        if b == n {
            hs_n.insert(a);
        }
    }
    println!(
        "{}",
        if hs_1.intersection(&hs_n).collect::<Vec<&i32>>().is_empty() {
            "IMPOSSIBLE"
        } else {
            "POSSIBLE"
        }
    );
}
