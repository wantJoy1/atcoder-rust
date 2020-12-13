use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main () {

  input! {
    n: i32,
    ss: [String; n]
  }

  // btreemapはkeyでソートされている
  let mut map: BTreeMap<String,i32> = BTreeMap::new();
  for s in ss {
    *map.entry(s).or_insert(0) += 1;
  }
  let mut max = 0;
  for (_,v) in &map {
    max = max.max(*v);
  }

  // btreemapはkeyでソートされている(2)
  for (k,v) in map.into_iter() {
    if v==max {
      println!("{}", k);
    }
  }

}