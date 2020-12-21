use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        uf.union(a - 1, b - 1);
    }
    let mut v = uf.into_labeling();
    v.sort();
    v.dedup();
    println!("{}", v.len() - 1);
}
