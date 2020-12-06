use proconio::input;

fn main () {

  input! {
    n: usize,
    d: [usize; n]
  }

  let mut sum = 0;
  for i in 0..n-1 {
    for j in i+1..n {
      sum += d[i]*d[j]
    }
  }
  println!("{}", sum);

}