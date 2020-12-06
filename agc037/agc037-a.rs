use proconio::input;

fn main () {

  input! {
    s: String
  }

  let mut chars = s.chars();
  let mut count = 1;
  let mut stack = vec![chars.next().unwrap()];

  loop {
    let next1 = match chars.next() {
      Some(v) => {v},
      None => {break;}
    };
    if stack.len()==1 && stack[0]==next1 {        
      let next2 = match chars.next() {
        Some(v) => {v},
        None => {break;}
      };
      stack.push(next2);
      count += 1;
      continue;
    } else {
      stack.clear();
      stack.push(next1);
      count += 1;
    }
  }

  println!("{}", count);

}