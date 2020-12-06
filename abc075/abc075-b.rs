use proconio::input;
use proconio::marker::Chars;

fn main () {

  input! {
    h: i32,
    w: i32,
    chars: [Chars; h]
  }

  let dx = [0, 1, 1,  1,  0, -1, -1, -1];
  let dy = [1, 1, 0, -1, -1, -1,  0,  1];

  for i in 0..h {
    for j in 0..w {

      let after: char = if chars[i][j] == '.' {
        let mut bomb: u32 = 0;
        for k in 0..8 {
          bomb += match chars[i+dx[k]][j+dy[k]] {
            Some('#') => 1,
            _ => 0
          };
        };
        std::char::from_digit(bomb)
      } else {'#'};

      chars[i][j] = after;
      
    }
  }

  for i in 0..h {
    println!("{}", chars[w*i..w*i+w]);
  }

}