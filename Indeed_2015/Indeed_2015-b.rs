use proconio::{
  fastout, input,
  marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::VecDeque;

fn main () {

  input! {
    s: Chars,
    t: Chars
  }

  let mut vs = s.iter().collect::<VecDeque<_>>();
  let vt = t.iter().collect::<VecDeque<_>>();
  println!("vs: {:?}",vs);
  println!("vt: {:?}",vt);

  for i in 0..=s.len() {
    if vs == vt {
      println!("{}", i);
      return;
    }
    let z = vs.pop_back().unwrap();
    println!("z: {}", z);
    vs.push_front(z);
  }
  println!("-1");
  
  /*
  if s.len() != t.len() {
    println!("-1");
    return;
  }

  if s == t {
    println!("0");
    return;
  }

  let repeated = format!("{}{}",s,s);
  let mut index = 0;
  let mut found = false;
  for i in 0..s.len()*2-t.len() {
    let slice = &repeated[i..i+t.len()];
    if slice == t {
      index = i;
      found = true;
      println!("{}",slice == t);
    }
  };
  if found {
    println!("{}",t.len()-index);
    return;
  } else {
    println!("-1");
    return;
  }
  */

}