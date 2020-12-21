use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut v: Vec<i32> = vec![0; 10];
    for i in s.chars() {
        let j = i as usize - 48;
        v[j] += 1;
    }

    let mut flag = false;
    if s.len() < 4 {
        let perms = s.chars().permutations(s.len());
        for perm in perms {
            let n: i32 = perm.into_iter().collect::<String>().parse().unwrap();
            if n % 8 == 0 {
                flag = true;
            }
        }
    } else {
        let multi_8 = (100..1000).into_iter().filter(|i| i % 8 == 0);
        for m8 in multi_8 {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for ch in m8.to_string().chars() {
                *map.entry(ch as i32 - 48).or_insert(0) += 1;
            }
            if map.into_iter().all(|(key, val)| v[key as usize] >= val) {
                flag = true;
            }
        }
    }

    println!("{}", if flag { "Yes" } else { "No" });
}
