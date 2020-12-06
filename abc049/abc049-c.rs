use proconio::input;

fn main () {

  input! {
    s: String
  }

  let patterns = ["eraser","erase","dreamer","dream"];
  let r = patterns.iter().fold(s, |t,x| t.replace(x, ""));
  println!("{}", match r.is_empty() {
    true => "YES",
    _    => "NO"
  });

}