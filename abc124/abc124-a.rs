use proconio::input;

fn main () {

  input! {
    a: u16,
    b: u16
  }

  let mut v = vec![a, b, a.max(b)-1];
  v.sort();
  println!("{}", v[1]+v[2]);

  // official
  println!("{}", if a>b {a+(a-1)}
            else if a<b {b+(b-1)}
            else {a+b});
}