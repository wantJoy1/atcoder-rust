use proconio::input;

fn main () {

  input! {
    mut abc: [u8; 3]
  }

  abc.sort();
  abc.dedup();
  println!("{}", if abc.len()==2 {"Yes"} else {"No"});

}