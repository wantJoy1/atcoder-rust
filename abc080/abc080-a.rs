use proconio::input;

fn main () {

  input! {
    n: u16,
    a: u16,
    b: u16
  }

  println!("{}", (a*n).min(b));
}