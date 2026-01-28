use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        xy: [(i64, i64); n],
        ab: [(Usize1, Usize1); q],
    }

    let mut sorted_points = xy.clone();
    sorted_points.sort_unstable_by(|&x, &y| compare_by_angle(x, y).reverse());

    let solve = |a: usize, b: usize| {
        let lower = sorted_points
            .partition_point(|&point| compare_by_angle(point, xy[a]).reverse().is_lt());
        let upper = sorted_points
            .partition_point(|&point| compare_by_angle(point, xy[b]).reverse().is_le());
        upper + n * (lower >= upper) as usize - lower
    };

    let output = ab.iter().map(|&(a, b)| solve(a, b)).join("\n");
    println!("{output}");
}

/// xy平面上の2つの格子点の偏角(x軸正方向を基準とした反時計回りの角度)を比較します。
pub fn compare_by_angle(point1: (i64, i64), point2: (i64, i64)) -> std::cmp::Ordering {
    let is_upper_half = |point: (i64, i64)| point.1 > 0 || (point.1 == 0 && point.0 > 0);
    match (is_upper_half(point1), is_upper_half(point2)) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => (point2.0 * point1.1).cmp(&(point1.0 * point2.1)),
    }
}
