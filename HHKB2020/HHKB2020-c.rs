use proconio::input;

fn main () {

  input! {
    n: i32,
    p: [u32; n]
  }

  let mut v: Vec<bool> = Vec::new();
  for i in 1..=200010 {
    v.push(false);
  }

  let mut at = 0;

  for i in p {
    v[i as usize] = true;
    while v[at] {
      at += 1;
    }
    println!("{}", at);
  }
  

}