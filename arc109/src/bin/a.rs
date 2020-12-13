use proconio::input;

fn main () {

  input! {
    a: i32,
    b: i32,
    x: i32,
    y: i32,
  }

  let way = (2*x).min(y);
  let d = (2*b+1-2*a).abs();
  let ans = (d/2)*way+x;

  println!("{}", ans);

}