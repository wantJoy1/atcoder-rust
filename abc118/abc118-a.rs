use proconio::input;

fn main () {

  input! {
    a: i16,
    b: i16,
  }

  println!("{}", if b%a==0 {a+b} else {b-a});

}