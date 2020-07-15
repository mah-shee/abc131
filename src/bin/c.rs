use num_integer::gcd;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let gcd = gcd(c, d);
    let lcm = c * d / gcd;
    let b_ans = b - b / c - b / d + b / lcm;
    let a_ans = (a - 1) - (a - 1) / c - (a - 1) / d + (a - 1) / lcm;
    let ans = b_ans - a_ans;
    println!("{:?}", ans);
}
