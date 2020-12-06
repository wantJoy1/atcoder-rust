use proconio::input;

fn main () {

  input! {
    n: i32,
    s: [String; n]
  }

  let mut r = 0;
  let mut b = 0;
  for i in s {
    for j in i.chars().into_iter() {
      if j=='R' { r += 1 };
      if j=='B' { b += 1 };
    };
  };

  println!("{}", 
    if r>b {"TAKAHASHI"}
    else if r<b {"AOKI"}
    else {"DRAW"});

}