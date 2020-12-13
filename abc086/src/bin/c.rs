use proconio::input;

fn main () {

  input! {
    n: i32,
    txys: [[i32; 3]; n]
  }

  let mut ans = true;
  let mut at = (0,0);
  let mut t = 0;
  for txy in txys {
    let d = (at.0-txy[1]).abs()+(at.1-txy[2]).abs();
    let time = txy[0]-t;
    if d<=time && d%2==time%2 {
      t = txy[0];
      at = (txy[1], txy[2]);
    } else {
      ans = false;
    }
  }
  println!("{}", if ans {"Yes"} else {"No"});

}