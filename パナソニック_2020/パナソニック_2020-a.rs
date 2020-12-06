use proconio::input;

fn main () {

  input! {
    k: u8
  }

  let v = vec![1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4, 1, 51];
  println!("{}", v[(k as usize)-1]);

}