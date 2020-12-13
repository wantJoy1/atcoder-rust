use proconio::input;

fn main () {
  input! {
    a: u32,
    b: u32,
  }

  println!("{}", match (a*b)%2 {
    1 => "Odd",
    _ => "Even",
  });

}