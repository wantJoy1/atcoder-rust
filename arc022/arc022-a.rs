fn main() {
  let s = read::<String>();
  let t = "ICT";

  let t = t.chars().collect::<Vec<_>>();
  let mut i = 0;
  for c in s.to_uppercase().chars() {
    if c == t[i] {
      i += 1;
      if i == t.len() {break}
    }
  }
  
  println!("{}", if i == t.len() {"YES"} else {"NO"});
}