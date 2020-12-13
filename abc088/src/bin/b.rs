use proconio::input;

fn main () {

  input! {
    n: u16,
    mut a: [u16; n]
  }

  a.sort_by(|x,y| x.cmp(y).reverse());
  let mut alice = 0;
  let mut bob   = 0;
  for (i, &x) in a.iter().enumerate() {
    match i%2 {
      0 => { alice += x; }
      _ => { bob   += x; }
    }
  }
  println!("{}",alice-bob);

}