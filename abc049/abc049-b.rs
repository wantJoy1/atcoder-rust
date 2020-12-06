use proconio::input;

fn main () {

  input! {
    h: i32,
    w: i32,
    c: [String; h]
  }

  for s in c {
    println!("{}", s);
    println!("{}", s);
  }

}