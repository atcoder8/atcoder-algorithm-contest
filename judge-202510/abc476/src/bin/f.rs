// unfinished

use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut imos = vec![vec![0; 2 * n]; 2 * n];
    for (i, j) in iproduct!(0..n, 0..n) {
        imos[i + j][n + i - j] = aa[i] * bb[j] % m;
    }
    for i in 0..2 * n {
        for j in 0..2 * n - 1 {
            imos[i][j + 1] += imos[i][j];
        }
    }
    for j in 0..2 * n {
        for i in 0..2 * n - 1 {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        (imos[top][left] + imos[bottom][right]) - (imos[top][right] + imos[bottom][left])
    };

    let init_cost = iproduct!(0..n, 0..n)
        .map(|(r, c)| aa[r] * bb[c] % m * (r + c) + (n + r - c))
        .sum::<usize>();

    let mut cost_array = vec![vec![0; 2 * n]; 2 * n];
    cost_array[0][n] = init_cost;
    for row in 1..2 * n {
        cost_array[row + 1][n] = cost_array[row][n] + calc_rect_sum(0, 1, 0, 2 * n - 1)
            - calc_rect_sum(1, 2 * n - 1, 0, 2 * n - 1);
    }
    for row in 0..2 * n - 1 {
        for col in (0..n).rev() {
            cost_array[row][col] = cost_array[row][col + 1] - calc_rect_sum(0, 2 * n - 1, 0, col)
                + calc_rect_sum(0, 2 * n - 1, col, 2 * n - 1);
        }
        for col in n + 1..2 * n - 1 {
            cost_array[row][col] = cost_array[row][col - 1] + calc_rect_sum(0, 2 * n - 1, 0, col)
                - calc_rect_sum(0, 2 * n - 1, col, 2 * n - 1);
        }
    }

    let xor = iproduct!(0..n, 0..n).fold(0_usize, |acc, (i, j)| {
        acc ^ (cost_array[i + j][n + i - j] * i * n + j)
    });
    println!("{xor}");
}
