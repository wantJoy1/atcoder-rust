use proconio::input;
use std::collections::HashMap;

fn main () {

  input! {
    n: i32,
    ss: [String; n]
  }

  let mut map: HashMap<String, usize> =
    HashMap::new();
  let mut m_cnt = 0;
  for s in ss {
    *map.entry(s).or_insert(0) += 1;
  }
  for (_, v) in map.clone() {
    m_cnt = m_cnt.max(v);
  }
  for (k, v) in map.iter() {
    if m_cnt == *v {
      println!("{}", k);
    }
  }

}