use proconio::input;

fn main () {

  input! {
    H: i32,
    W: i32,
    h: i32,
    w: i32,
  }

  println!("{}", H*W-(h*W+w*H-h*w));

}