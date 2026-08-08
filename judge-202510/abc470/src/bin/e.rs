// unfinished

use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        aa: [u32; n],
    }

    let exp = recursion(l, n, 0, &mut vec![vec![vec![None; n + 1]; n + 1]; l + 1]);
    let ans = aa.iter().sum::<u32>() as f64 / n as f64 * exp;
    println!("{ans}");
}

fn recursion(life: usize, zero: usize, one: usize, memo: &mut [Vec<Vec<Option<f64>>>]) -> f64 {
    if life == 0 {
        return 0.0;
    }

    if zero == 0 {
        return one as f64;
    }

    if let Some(exp) = memo[life][zero][one] {
        return exp;
    }

    let mut exp = 0.0;

    // 失敗
    if zero >= 2 {
        exp += recursion(life - 1, zero - 2, one + 2, memo)
            * (zero as f64 / (zero + one) as f64)
            * (((2 * zero - 1) as f64) / (2 * (zero + one) - 1) as f64);
    }

    // 成功 (既知のカードを含む)
    if one >= 1 {
        exp += (1.0 + recursion(life, zero, one - 1, memo)) * (one as f64 / (zero + one) as f64);
    }

    // 成功 (既知のカードを含まない)
    if zero >= 1 {
        exp += (1.0 + recursion(life, zero - 1, one, memo))
            * (zero as f64 / (zero + one) as f64)
            * (1.0 / (2 * (zero + one) - 1) as f64);
    }

    // 失敗の後成功
    if zero >= 1 && one >= 1 {
        let add_score = if life >= 2 { 1.0 } else { 0.0 };
        exp += (add_score + recursion(life - 1, zero - 1, one, memo))
            * (zero as f64 / (zero + one) as f64)
            * ((2 * one) as f64 / (2 * (zero + one) - 1) as f64);
    }

    memo[life][zero][one] = Some(exp);

    exp
}
