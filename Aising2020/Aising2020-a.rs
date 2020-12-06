use proconio::input;

fn main () {

  input! {
    l: i32,
    r: i32,
    d: i32,
  }

  let mut ans = 0;
  for i in l..=r {
    if i%d==0 {
      ans += 1;
    }
  }
  println!("{}", ans);

  // range
  println!("{}", (l..=r).filter(|x| x%d==0).count());
  // N(1)
  println!("{}", (r/d)-(l-1)/d);

}