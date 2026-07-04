use itertools::{Itertools, enumerate};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut aa = vec![0; n];
    let mut reversed = false;
    let mut left = 0;
    let mut right = 0;
    for (i, &ch) in enumerate(&s).rev() {
        if ch == 'o' {
            reversed = !reversed;
        }

        if reversed {
            aa[n - 1 - i - right] = i + 1;
            left += 1;
        } else {
            aa[i + left] = i + 1;
            right += 1;
        }
    }

    println!("{}", aa.iter().join(" "));
}
