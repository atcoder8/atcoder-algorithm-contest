use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [Usize1; n],
        bb: [Usize1; n],
    }

    let Some(left) = (0..n).find(|&i| aa[i] != bb[i]) else {
        return true;
    };
    let Some(right) = (left + 1..n).rev().find(|&i| aa[i] != bb[i]) else {
        return false;
    };

    let mut shifted1 = aa.clone();
    shifted1.remove(left);
    shifted1.insert(right, aa[left]);

    let mut shifted2 = aa.clone();
    shifted2.remove(right);
    shifted2.insert(left, aa[right]);

    shifted1 == bb || shifted2 == bb
}
