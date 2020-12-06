use proconio::input;

fn main () {

  input! {
    n: u32,
    t: [u32; n],
    m: u32,
    px: [(u32,u32); m]
  }

  let sum = {
    let mut tmp = 0;
    for i in 0..n {
      tmp += t[i];
    }
    tmp
  };
  for (p,x) in &px[..] {
    let decrease = t[p-1]-x;
    println!("{}", sum-decrease);
  };

}