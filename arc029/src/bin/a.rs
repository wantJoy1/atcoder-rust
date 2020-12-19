use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }

    // println!("{}", go(0, 0, 0, n, &t));

    /*
    struct Closure<'s> {
        f: &'s Fn(&Closure, usize, usize, usize) -> usize,
    }
    let closure = Closure {
        f: &|closure, i, lsum, rsum| {
            if i == n {
                lsum.max(rsum)
            } else {
                let j = t[i];
                closure.f(closure, i + 1, lsum + j, rsum).min(closure.f(
                    closure,
                    i + 1,
                    lsum,
                    rsum + j,
                ))
            }
        },
    };
    println!("{}", closure(closure, 0, 0, 0));
    */

    /* power(lol)
    let mut v = vec![];
    if n == 1 {
        v.push(t[0]);
    }
    if n == 2 {
        v.push(t[0].max(t[1]));
    }
    if n == 3 {
        v.push(t[0].max(t[1] + t[2]));
        v.push(t[1].max(t[0] + t[2]));
        v.push(t[2].max(t[0] + t[1]));
    }
    if n == 4 {
        v.push(t[0].max(t[1] + t[2] + t[3]));
        v.push(t[1].max(t[0] + t[2] + t[3]));
        v.push(t[2].max(t[0] + t[1] + t[3]));
        v.push(t[3].max(t[0] + t[1] + t[2]));
        v.push((t[0] + t[1]).max(t[2] + t[3]));
        v.push((t[0] + t[2]).max(t[1] + t[3]));
        v.push((t[0] + t[3]).max(t[1] + t[2]));
    }
    println!("{}", v.into_iter().min().unwrap());
    */
}

fn go(i: usize, lsum: usize, rsum: usize, n: usize, t: &Vec<usize>) -> usize {
    if i == n {
        lsum.max(rsum)
    } else {
        go(i + 1, lsum + t[i], rsum, n, &t).min(go(i + 1, lsum, rsum + t[i], n, &t))
    }
}
