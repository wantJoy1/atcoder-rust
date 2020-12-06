use proconio::input;

fn main () {

  input! {
    n: i32,
    a: [i32; n]
  }

  let mut count = 1;
  for i in a {
    if count==i {
      count += 1;
    }
  }
  let ans = if count==1 {
    -1
  } else {
    n-(count-1)
  };
  println!("{}", ans);
  

}