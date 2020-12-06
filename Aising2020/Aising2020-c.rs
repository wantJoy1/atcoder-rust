use proconio::input;

fn main () {

  input! {
    n: u32
  }

  for i in 1..=n {
    let upper = (i as f64).sqrt() as u32;
    let mut count = 0;
    for x in 1..=upper {
      for y in 1..=upper {
        for z in 1..=upper {
          if x*x+y*y+z*z+x*y+y*z+z*x==i {
            count += 1;
          }
        }
      }
    }
    println!("{}", count);
  }

}