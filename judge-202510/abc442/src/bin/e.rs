use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

use crate::amplitude::Amplitude;

fn main() {
    input! {
        (n, q): (usize, usize),
        xy: [(i64, i64); n],
        ab: [(Usize1, Usize1); q],
    }

    // 時計回りに合わせるために左右反転
    let amplitudes = xy.iter().map(|&(x, y)| Amplitude::new(-x, y)).collect_vec();

    let mut sorted_amplitudes = amplitudes.clone();
    sorted_amplitudes.sort_unstable();

    let solve = |a: usize, b: usize| {
        let lower = sorted_amplitudes.lower_bound(&amplitudes[a]);
        let upper = sorted_amplitudes.upper_bound(&amplitudes[b]);
        upper + n * (lower >= upper) as usize - lower
    };

    let output = ab.iter().map(|&(a, b)| solve(a, b)).join("\n");
    println!("{output}");
}

pub mod amplitude {
    fn gcd(a: i64, b: i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();

        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }

        a
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Amplitude {
        x: i64,
        y: i64,
    }

    impl PartialOrd for Amplitude {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Amplitude {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let is_upper_half =
                |amplitude: Amplitude| amplitude.y > 0 || (amplitude.y == 0 && amplitude.x > 0);

            match (is_upper_half(*self), is_upper_half(*other)) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => (other.x * self.y).cmp(&(self.x * other.y)),
            }
        }
    }

    impl Amplitude {
        pub fn new(x: i64, y: i64) -> Self {
            assert_ne!((x, y), (0, 0));

            let g = gcd(x, y);
            Self { x: x / g, y: y / g }
        }
    }
}
