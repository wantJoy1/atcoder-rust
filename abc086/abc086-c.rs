use proconio::input;

fn main () {

  input! {
    n: i32,
    mut txy: [[i32; 3]; n]
  }

  txy.insert(0,[0,0,0].to_vec());
  let ans = txy[..].windows(2).all(|w| {
    let time = w[1][0]-w[0][0];
    let dist = (w[0][1]-w[1][1]).abs() + (w[0][2]-w[1][2]).abs();
    dist <= time && dist%2 == time%2
  });
  println!("{}", match ans {
    true => "Yes",
    _ => "No"
  });

}