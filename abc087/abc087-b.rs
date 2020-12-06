use proconio::input;

fn main () {

  input! {
    a: u16,
    b: u16,
    c: u16,
    x: u16
  }

  let mut count: u16 = 0;
  for i in 0..a+1 {
    for j in 0..b+1 {
      for k in 0..c+1 {
        if i*500+j*100+k*50==x {
          count += 1;
        }
      }
    }
  }
  println!("{}",count);


}