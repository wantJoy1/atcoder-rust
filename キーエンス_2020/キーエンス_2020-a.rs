use proconio::input;

fn main () {

  input! {
    h: f32,
    w: f32,
    n: f32
  }

  println!("{}",
    (if h<w {
      n/w
    } else {
      n/h
    }).ceil() as u16);

}