use proconio::input;

fn main () {

  input! {
    x: u8,
    a: u8
  }

  println!("{}", if x<a {0} else {10});

}