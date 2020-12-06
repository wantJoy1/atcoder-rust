use proconio::input;

fn main () {

  input! {
    n: i32,
    a: [i64; n]
  }

  let sum = &a.iter().sum();
  let mut tmp = 0i64;
  let mut sums: Vec<i64> = vec![];
  for i in &a[..] {
    tmp += i;
    sums.push((tmp-(sum-tmp)).abs());
  }
  let ans = sums.into_iter().min().unwrap();
  println!("{}", ans);

}