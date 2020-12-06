use proconio::input;
use num::integer::gcd;

fn main () {

  input! {
    n: i32,
    mut a: [i32; n]
  }

  while a.len() != 1 {
    a.sort();
    a.dedup();
    for i in 1..a.len() {
      let after = a[i]%a[0];
      a[i] = if after==0 {a[0]} else {after};
    }
  }

  println!("{}", a[0]);

}