use proconio::input;

fn main () {

  input! {
    s: String
  }

  for c in s.chars() {
    print!("{}", match c {
      'O' | 'D' => '0',
      'I' => '1',
      'Z' => '2',
      'S' => '5',
      'B' => '8',
      _   => c
    });
  }
  println!("");

}