use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        q: i32,
        abcd: [(i32, i32, i32, i32); q]
    }

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for (a, b, c, d) in abcd {}
    let ans = map.values().sum::<i32>();
    println!("{}", ans);
}
