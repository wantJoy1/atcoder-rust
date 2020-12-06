use proconio::input;

fn main () {

  input! {
    a: u8,
    b: u8
  }

  let mut v = vec![1,2,3];
  v.retain(|&i| i != a && i != b);
  println!("{}", v[0]);

}