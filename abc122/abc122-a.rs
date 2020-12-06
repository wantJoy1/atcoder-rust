use proconio::input;

fn main () {

  input! {
    b: char
  }

  println!("{}", match b {
    'A' => 'T',
    'C' => 'G',
    'G' => 'C',
    _   => 'A'
  });

}