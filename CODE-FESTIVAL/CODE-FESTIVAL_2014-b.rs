use proconio::input;

fn main () {

  input! {
    s: String
  }

  let mut sum = 0;
  for (i,c) in s.chars().enumerate() {
    let add = c as i32 - '0' as i32;
    sum += if i%2==0 {add} else {add*(-1)};
  }
  println!("{}", sum);
  
}