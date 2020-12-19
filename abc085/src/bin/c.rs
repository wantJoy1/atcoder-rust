use proconio::input;

fn main() {
  input! {
    n: i64,
    y: i64
  }

  let mut ans: (i64, i64, i64) = (-1, -1, -1);
  'outer: for i in 0..=n {
    for j in 0..=n - i {
      let k = n - i - j;
      if 10000 * i + 5000 * j + 1000 * k == y {
        ans = (i, j, k);
        break 'outer;
      }
    }
  }
  println!("{} {} {}", ans.0, ans.1, ans.2);
}
