use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n],
    }

    let mut acc_min = vec![n; n + 1];
    for &(x, y) in &xy {
        acc_min[x + 1] = y;
    }
    for i in 0..n {
        acc_min[i + 1] = acc_min[i + 1].min(acc_min[i]);
    }

    let cnt = xy.iter().filter(|&&(x, y)| acc_min[x] >= y).count();
    println!("{cnt}");
}
