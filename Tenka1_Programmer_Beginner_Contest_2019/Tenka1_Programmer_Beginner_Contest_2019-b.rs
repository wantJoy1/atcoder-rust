use proconio::input;

fn main () {

  input! {
    n: usize,
    s: String,
    k: usize
  }

  let ch = s.chars().nth(k-1).unwrap();
  let ans = s.chars().map(|c| {
    if c==ch {
      c
    } else {
      '*'
    }
  }).collect::<String>();
  println!("{}",ans);

}