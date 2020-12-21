use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        s: [String; n]
    }

    let s = s.into_iter().map(|i| {
        let mut v: Vec<char> = i.chars().collect();
        v.sort();
        v.into_iter().collect::<String>()
    });
    let mut map: HashMap<String, i64> = HashMap::new();
    for i in s {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut ans: i64 = 0;
    for (_, v) in map {
        ans += (v * (v - 1)) / 2;
    }
    println!("{}", ans);
}
