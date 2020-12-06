use proconio::input;
use proconio::marker::Chars;

fn main () {

  input! {
    s: Chars,
    t: Chars
  }

  let ans = (0..3).filter(|&i|
    s.iter().nth(i) == t.iter().nth(i)
  ).count();
  println!("{}", ans);

}