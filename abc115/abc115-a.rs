use proconio::input;

fn main () {

  input! {
    d: u8
  }

  let ans = format!("Christmas{}",
    " Eve".repeat((25u8-d) as usize));

  println!("{}", ans);

}