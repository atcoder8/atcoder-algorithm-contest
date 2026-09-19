use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        (x, y): (usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; m],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut acc_a = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc_a[i + 1] = acc_a[i] + a;
    }
    acc_a.remove(0);

    let mut num_ones = x;
    let mut rem_k = y;

    let mut ans = acc_a.partition_point(|&prefix_sum| prefix_sum <= x + k * y);
    for (j, &b) in enumerate(&bb) {
        let req_k = b.div_ceil(k);

        if req_k > rem_k {
            break;
        }

        rem_k -= req_k;
        num_ones += k * req_k - b;

        ans = ans
            .max(j + 1 + acc_a.partition_point(|&prefix_sum| prefix_sum <= num_ones + k * rem_k));
    }

    println!("{ans}");
}
