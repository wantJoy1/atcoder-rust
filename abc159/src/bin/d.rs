use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    let mut map: HashMap<i64, i64> = HashMap::new();
    for &i in &a {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut sum: i64 = 0;
    for v in map.values() {
        sum += v * (v - 1) / 2;
    }

    for k in a {
        let v = map[&k];
        println!("{}", sum - v + 1);
    }
}
