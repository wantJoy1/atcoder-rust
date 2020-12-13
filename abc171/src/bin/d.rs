use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    
    input!{
        n: u32,
        a: [i64; n],
        q: u32,
        bc: [(i64,i64); q]
    }

    // key -> cnt
    let mut map: HashMap<i64,i64> = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut sum: i64 = 0;
    for (&k,&v) in &map {
        sum += k*v ;
    }

    for (b,c) in bc {
        let b_cnt = if map.contains_key(&b) {
            map[&b]
        } else {0};
        sum = sum+(c-b as i64)*b_cnt;
        println!("{}", sum);
        map.remove(&b);
        *map.entry(c).or_insert(0) += b_cnt;
    }

}
