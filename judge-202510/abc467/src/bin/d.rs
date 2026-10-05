use itertools::Itertools;
use num::Rational64;
use proconio::input;

use crate::Solution::*;

fn main() {
    input! {
        t: usize,
        coords_vec: [[(i64, i64); 4]; t],
    }

    let output = coords_vec
        .iter()
        .map(|coords| if solve(coords) { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Solution {
    Impossible,
    Indeterminate,
    Determine(Rational64),
}

fn calc_slope(coord1: (i64, i64), coord2: (i64, i64)) -> Solution {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;

    if x1 == x2 {
        if y1 == y2 { Indeterminate } else { Impossible }
    } else {
        Determine(Rational64::new(y1 - y2, x1 - x2))
    }
}

fn solve(coords: &[(i64, i64)]) -> bool {
    let (px, py) = coords[0];
    let (qx, qy) = coords[1];
    let (rx, ry) = coords[2];
    let (sx, sy) = coords[3];

    let sol1 = calc_slope((px, rx), (qx, sx));
    let sol2 = calc_slope((py, ry), (qy, sy));

    // PQ と RS が並行でない場合は PQ, RS それぞれに対する垂直二等分線が共有点を持つ
    if matches!(sol1, Impossible)
        || matches!(sol2, Impossible)
        || matches!((sol1, sol2), (Determine(ratio1), Determine(ratio2)) if ratio1 != ratio2)
    {
        return true;
    }

    // PQ と RS が並行である場合、それらの垂直二等分線も平行であるから、PQ の中点を M、RS の中点を N とすると、以下の4つの条件は同値である
    // - PQ, RS それぞれに対する垂直二等分線が共有点を持つ
    // - PQ, RS それぞれに対する垂直二等分線が一致する
    // - M と N が一致する、または M と N を結ぶ線分 と PQ が垂直である
    // - P, Q, M, N の位置ベクトルをそれぞれ p, q, m, n とすると、m - n と p - q の内積は 0 である
    let (mx, my) = (px + qx, py + qy); // 2m を計算
    let (nx, ny) = (rx + sx, ry + sy); // 2n を計算
    // 2(m - n) と (p - q) の内積が 0 であるかどうかを判定
    (mx - nx) * (px - qx) + (my - ny) * (py - qy) == 0
}
