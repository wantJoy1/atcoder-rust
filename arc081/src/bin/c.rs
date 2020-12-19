use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i64; n]
    }

    let mut count: HashMap<i64, i32> = HashMap::new();
    for i in a {
        *count.entry(i).or_insert(0) += 1;
    }
    let v: Vec<(i64, i32)> = count.into_iter().filter(|kv| kv.1 >= 2).collect();
    if v.len() == 0 || (v.len() == 1 && v[0].1 < 4) {
        println!("0");
    } else if v.len() == 1 {
        println!("{}", v[0].0 * v[0].0);
    } else {
        let mut sorted = v;
        sorted.sort_by_key(|kv| kv.0);
        sorted.reverse();
        let sorted = sorted;
        if sorted[0].1 >= 4 {
            println!("{}", sorted[0].0 * sorted[0].0);
        } else {
            println!("{}", sorted[0].0 * sorted[1].0);
        }
    }
}
