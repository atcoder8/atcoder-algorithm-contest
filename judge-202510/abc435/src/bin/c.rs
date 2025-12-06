use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut max = 1;
    let ans = enumerate(&aa)
        .take_while(|&(i, &a)| {
            if i >= max {
                return false;
            }

            max = max.max(i + a);
            true
        })
        .count();
    println!("{ans}");
}
