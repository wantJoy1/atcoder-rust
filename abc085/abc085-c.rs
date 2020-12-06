use proconio::input;

fn main () {

  input! {
    n: i64,
    y: i64
  }

  let mut ijk: Vec<i64> = vec![-1,-1,-1];
  'outer: for i in 0..=n {
    for j in 0..=n-i {
      let k = n-i-j;
      if 10000*i+5000*j+1000*k == y {
        ijk[0] = i;
        ijk[1] = j;
        ijk[2] = k;
        break 'outer;
      }
    }
  }
  println!("{} {} {}", ijk[0], ijk[1], ijk[2]);

}