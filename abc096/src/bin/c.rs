use proconio::input;
fn main() {
    
    input!{
        h: usize,
        w: usize,
        const ss: [String; h]
    }


    let mut can = true;
    for i in 0..h {
        for j in 0..w {
            match &ss[i].chars().nth(j).unwrap();
            if *c=='#' {
                if !is_drawable(ss, i, j) {
                    can = false;
                }
            }

        }
    }
    println!("{}", if can {"Yes"} else {"No"});

}

fn is_drawable(ss: &Vec<String>, i: usize, j: usize) -> bool {
    let d = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut tmp = false;
    for (x,y) in d {
        match ss(x).chars().nth(y) {
            Some(c) => {
                if c=='#' {tmp = true;};
            },
            None => ()
        };
    }
    tmp
}