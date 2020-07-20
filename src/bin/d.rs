#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut time: [(usize, usize); n]
    }
    time.sort_by(|a, b| a.1.cmp(&b.1));
    let mut a = 0;
    for i in 0..n {
        a += time[i].0;
        if a > time[i].1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
