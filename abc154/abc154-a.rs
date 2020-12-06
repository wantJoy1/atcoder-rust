use proconio::input;

fn main () {

  input! {
    s: String,
    t: String,
    mut a: i8,
    mut b: i8,
    u: String,
  }

  if s == u {
    a -= 1;
  } else {
    b -= 1;
  }
  
  println!("{} {}", a, b);

}