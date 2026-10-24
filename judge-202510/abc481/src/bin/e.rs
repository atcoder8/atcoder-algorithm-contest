use itertools::Itertools;
use num::Rational64;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }

    let sub_solve = |xy: &[(i64, i64)]| {
        let mut first_quadrant = vec![];
        let mut num_second_quadrant_points = 0;
        let mut third_quadrant = vec![];
        for &(x, y) in xy {
            if x > 0 && y >= 0 {
                first_quadrant.push(Rational64::new(y, x));
            } else if y > 0 && x <= 0 {
                num_second_quadrant_points += 1;
            } else if x < 0 && y <= 0 {
                third_quadrant.push(Rational64::new(y, x));
            }
        }
        first_quadrant.sort_unstable_by_key(|&slope| slope);
        third_quadrant.sort_unstable_by_key(|&slope| slope);

        let mut max_score = 0;
        let mut num_out_points = 0;
        let mut num_in_points = 0;
        while num_out_points < first_quadrant.len() {
            let slope = first_quadrant[num_out_points];
            while num_in_points < third_quadrant.len() && third_quadrant[num_in_points] < slope {
                num_in_points += 1;
            }
            let num_one_side_points =
                first_quadrant.len() - num_out_points + num_second_quadrant_points + num_in_points;
            max_score = max_score
                .max(num_one_side_points)
                .max(n - num_one_side_points);
            while num_out_points < first_quadrant.len() && first_quadrant[num_out_points] == slope {
                num_out_points += 1;
            }
        }

        max_score
    };

    (0..4)
        .map(|_| {
            xy.iter_mut().for_each(|(x, y)| (*x, *y) = (*y, -*x));
            sub_solve(&xy)
        })
        .max()
        .unwrap()
}
