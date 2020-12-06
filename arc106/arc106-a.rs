use proconio::input;
use std::process;

fn main () {

  input! {
    n: u128
  }

  for i in 1..38 {
    for j in 1..26 {
      if 3u128.pow(i)+5u128.pow(j)==n {
        println!("{} {}", i, j);
        process::exit(0);
      }
    }
  }
  println!("-1");

}