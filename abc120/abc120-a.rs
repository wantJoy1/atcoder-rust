use proconio::input;

fn main () {

  input! {
    a: u16,
    b: u16,
    c: u16,
  }

  println!("{}", (b/a).min(c));

}