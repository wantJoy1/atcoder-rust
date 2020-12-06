use proconio::input;

fn main () {

  input! {
    n: i32,
    m: i32,
    mut ab: [[i32; 2]; n]
  }

  let mut = ab.sort_by(|a,b| a[0] < b[0]).unwrap());


}