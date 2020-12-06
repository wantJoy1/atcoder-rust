use proconio::input;

fn main () {

  input! {
    s: String
  }

  let w = vec!["SUN","MON","TUE","WED","THU","FRI","SAT"];
  let ans = 7 - w.iter().position(|&i| i == s).unwrap();
  println!("{}",ans);

}