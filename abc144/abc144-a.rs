use proconio::input;

fn main () {

  input! {
    a: i32,
    b: i32
  }

  println!("{}", if 1<=a&&a<=9 && 1<=b && b<=9 {
    a*b
  } else {
    -1
  });

}