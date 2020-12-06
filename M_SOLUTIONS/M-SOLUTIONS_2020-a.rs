use proconio::input;

fn main () {

  input! {
    x: u16
  }

  println!("{}", 8-(x-400)/200);

  /*
  println!("{}",
         if x<=599 {8}
    else if x<=799 {7}
    else if x<=999 {6}
    else if x<=1199 {5}
    else if x<=1399 {4}
    else if x<=1599 {3}
    else if x<=1799 {2}
    else {1}
  );
  */

}