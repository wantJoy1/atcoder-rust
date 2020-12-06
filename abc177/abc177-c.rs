use proconio::input;

fn main () {

  input! {
    n: u32,
    a: [u64; n]
  }

  let m = 1_000_000_000u64+7;
  let mut sum: u64 = (a[0]*a[1])%m;
  let mut tmp: u64 = 0;
  for (x,i) in &a[..n-2].enumerate() {
    tmp
  }

}