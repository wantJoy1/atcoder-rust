use proconio::input;

fn main () {

  input! {
    a: [u8; 3]
  }

  let sum: u8 = a.iter().sum();
  println!("{}", if sum >= 22 {"bust"} else {"win"});

}