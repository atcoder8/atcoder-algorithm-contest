use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let is_ok = |a1: u128, len: u128, d: u128| {
        let mut prod = 1;
        (0..len).all(|k| {
            prod *= a1 + d * k;
            prod <= n
        })
    };

    let mut cnt = 0_u128;

    for a1 in 1.. {
        if a1 * (a1 + 1) * (a1 + 2) > n {
            break;
        }

        for len in 3.. {
            if !is_ok(a1, len, 1) {
                break;
            }

            let mut ok = 1_u128;
            let mut ng = n.sqrt();
            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;
                if is_ok(a1, len, mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            cnt += ok;
        }
    }

    println!("{cnt}");
}
