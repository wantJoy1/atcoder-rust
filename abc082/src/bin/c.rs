use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k, v) in map {
        if k < v {
            ans += v - k;
        } else if k > v {
            ans += v;
        }
    }
    println!("{}", ans);
}
