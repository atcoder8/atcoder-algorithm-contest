// unfinished

use itertools::Itertools;
use proconio::input;

const MOD: usize = 10007;

fn main() {
    input! {
        (k, m): (usize, usize),
        cl: [(usize, usize); k],
    }

    let m2 = m.pow(2);

    let init = (0..m2).map(|i| (10 * i + 1) % m2).collect_vec();
    let mut doubling = vec![init];
    for exp in 0..30 {
        let current = &doubling[exp];
        let next = (0..m2).map(|i| current[current[i]]).collect_vec();
        doubling.push(next);
    }

    let calc_repunit_number_mod_m2 = |num_digits: usize| {
        let mut number = 0;
        for exp in 0..30 {
            if num_digits >> exp & 1 == 1 {
                number = doubling[exp][number];
            }
        }
        number
    };

    let mut n = 0;
    let mut n_mod_m2 = 0;
    for &(c, l) in &cl {
        n = (n * pow_mod(10, l, m2)) % m2;
        n = (n + (c * pow_mod(10, l, m2) + m2 - 1) % m2) % m2;
        n_mod_m2 = (n_mod_m2 * pow_mod(10, l, m2)) % m2;
        n_mod_m2 = (n_mod_m2 + c * calc_repunit_number_mod_m2(l) % m2) % m2;
    }

    let n_mod_m = n_mod_m2 / m % m;
    let ans = (n - n_mod_m) * modinv(m as i64, MOD as i64) as usize % MOD;
    println!("{ans}");
}

/// Calculate the remainder of `exp` power of `base` divided by `m`.
pub fn pow_mod(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1 % m;
    let mut mul = base % m;
    let mut t = exp;

    while t != 0 {
        if t & 1 == 1 {
            ret = ret * mul % m;
        }

        mul = mul * mul % m;
        t >>= 1;
    }

    ret
}

fn modinv(mut a: i64, m: i64) -> i64 {
    assert!(m >= 2);

    let (mut b, mut s, mut t) = (m, 1, 0);
    while b != 0 {
        let q = a / b;
        a -= q * b;
        std::mem::swap(&mut a, &mut b);
        s -= q * t;
        std::mem::swap(&mut s, &mut t);
    }

    assert_eq!(
        a.abs(),
        1,
        "The inverse does not exist. gcd(a, m) = {}",
        a.abs()
    );

    s %= m;
    if s < 0 {
        s += m;
    }

    s
}
