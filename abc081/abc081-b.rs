use proconio::input;

fn main () {

  input! {
    n: u16,
    a: [u32; n]
  }

  println!("{}",
    a.iter()
     .map(|&c| c.trailing_zeros())
     .min()
     .unwrap()
  );

}