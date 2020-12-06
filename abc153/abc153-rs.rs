use proconio::input;

fn main () {

  input! {
    h: f32,
    a: f32
  }

  println!("{}", (h/a).ceil() as i32);

}