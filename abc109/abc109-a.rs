use proconio::input;

fn main () {

  input! {
    a: i32,
    b: i32,
  }

  let ans = (1..=3).all(|i|
    (a*b*i)%2 == 0
  );
  println!("{}", if ans {"No"} else {"Yes"});

}