use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [u64; n],
    }

    let queries = (0..q).map(|_| Query::read());

    let mut prefix_sum = vec![0_u64; n + 1];
    for (i, &a) in enumerate(&aa) {
        prefix_sum[i + 1] = prefix_sum[i] + a;
    }

    for query in queries {
        match query {
            Query::Swap(x) => {
                prefix_sum[x + 1] -= aa[x];
                prefix_sum[x + 1] += aa[x + 1];
                aa.swap(x, x + 1);
            }
            Query::Output { l, r } => println!("{}", calc_interval_sum(&prefix_sum, l, r)),
        }
    }
}

fn calc_interval_sum(prefix_sum: &[u64], left: usize, right: usize) -> u64 {
    prefix_sum[right] - prefix_sum[left]
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Swap(usize),
    Output { l: usize, r: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                x: Usize1,
            }

            Self::Swap(x)
        } else {
            input! {
                (l, r): (Usize1, usize),
            }

            Self::Output { l, r }
        }
    }
}
