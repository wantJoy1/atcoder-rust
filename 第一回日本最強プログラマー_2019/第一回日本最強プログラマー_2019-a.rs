use proconio::input;

fn main () {

  input! {
    m: i32,
    d: i32
  }

  fn is_product(mon: i32, day: i32) -> bool {
    let day1  = day%10;
    let day10 = day/10;
    day1  >= 2 &&
    day10 >= 2 &&
    day1 * day10 == mon
  }

  let mut count = 0; 
  for i in 1..=m {
    for j in 1..=d {
      if is_product(i, j) {
        count += 1;
      }
    }
  }
  println!("{}", count);

}