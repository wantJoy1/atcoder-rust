use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        s: [String; n]
    }

    let mut map: HashMap<String, i32> = HashMap::new();
    for i in s {
        *map.entry(i).or_insert(0) += 1;
    }
    let ans = map
        .into_iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    println!("{}", ans.0);
}
