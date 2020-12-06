use proconio::input;

fn main () {

  input! {
    s: String
  }

  println!("{}", match s.as_str() {
    "Sunny"  => "Cloudy",
    "Cloudy" => "Rainy",
    _ => "Sunny"
  })

}