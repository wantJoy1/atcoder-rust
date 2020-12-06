use proconio::input;

fn main () {

  input! {
    n: i32,
    r: i32
  }

  println!("{}", if n>=10 {r} else {
    r+1000-100*n
  });

}