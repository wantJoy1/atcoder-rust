use proconio::input;

fn main () {

  input! {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
  }

  let l = a+b;
  let r = c+d;

  println!("{}", if l>r {"Left"}
            else if l<r {"Right"}
            else {"Balanced"});

}