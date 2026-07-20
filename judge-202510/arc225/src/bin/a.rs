// unfinished

use itertools::{Itertools, iproduct};
use ndarray::prelude::*;
use proconio::{input, marker::Usize1};

type Pattern = [u8; 4];

fn main() {
    input! {
        n: usize,
        xxx: [[Usize1; n]; n],
    }

    let mut dp = Array2::<Vec<Node>>::default([n - 1; 2]);
    for (row, col) in iproduct!(0..n - 1, 0..n - 1) {
        let candidate_patterns = (0_u8..=255).filter_map(|bits| {
            let pattern: [u8; 4] = std::array::from_fn(|i| (bits >> (2 * i)) & 3);
            if is_match_pattern(&xxx, row, col, pattern) {
                Some(pattern)
            } else {
                None
            }
        });

        let find_upper_pattern = |pattern: Pattern| {
            if row == 0 {
                return Some([0; 4]);
            }

            dp[(row - 1, col)]
                .iter()
                .copied()
                .find(|node| is_match_vertical(node.pattern, pattern))
                .map(|node| node.pattern)
        };

        let find_left_pattern = |pattern: Pattern| {
            if col == 0 {
                return Some([0; 4]);
            }

            dp[(row, col - 1)]
                .iter()
                .copied()
                .find(|node| is_match_horizontal(node.pattern, pattern))
                .map(|node| node.pattern)
        };

        let find_upper_left_pattern = |pattern: Pattern| {
            if row == 0 || col == 0 {
                return Some([0; 4]);
            };

            dp[(row - 1, col - 1)]
                .iter()
                .copied()
                .find(|node| is_match_diagonal(node.pattern, pattern))
                .map(|node| node.pattern)
        };

        dp[(row, col)] = candidate_patterns
            .filter_map(|pattern| {
                let upper_pattern = find_upper_pattern(pattern)?;
                let left_pattern = find_left_pattern(pattern)?;
                let upper_left_pattern = find_upper_left_pattern(pattern)?;
                Some(Node {
                    upper_pattern,
                    left_pattern,
                    upper_left_pattern,
                    pattern,
                })
            })
            .collect_vec();
    }

    let mut yyy = vec![vec![0; n]; n];
    let mut pattern_array = vec![vec![[0; 4]; n - 1]; n - 1];
    pattern_array[n - 2][n - 2] = dp[(n - 2, n - 2)][0].pattern;
    for row in (0..n - 1).rev() {
        for col in (0..n - 1).rev() {
            let node = *dp[(row, col)]
                .iter()
                .find(|node| node.pattern == pattern_array[row][col])
                .unwrap();
            if row > 0 {
                pattern_array[row - 1][col] = node.upper_pattern;
            }
            if col > 0 {
                pattern_array[row][col - 1] = node.left_pattern;
            }
            if row > 0 && col > 0 {
                pattern_array[row - 1][col - 1] = node.upper_left_pattern;
            }
            for (dr, dc) in iproduct!(0..2, 0..2) {
                yyy[row + dr][col + dc] = node.pattern[2 * dr + dc];
            }
        }
    }

    let output = yyy
        .iter()
        .map(|yy| yy.iter().map(|y| y + 1).join(" "))
        .join("\n");
    println!("{output}");
}

fn is_match_pattern(xx: &[Vec<usize>], row: usize, col: usize, pattern: Pattern) -> bool {
    iproduct!(0..2, 0..2).all(|(dr, dc)| {
        (dr == 1
            || (xx[row][col + dc].abs_diff(xx[row + 1][col + dc]) == 1)
                == (pattern[dc].abs_diff(pattern[2 + dc]) >= 2))
            && (dc == 1
                || (xx[row + dr][col].abs_diff(xx[row + dr][col + 1]) == 1)
                    == (pattern[2 * dr].abs_diff(pattern[2 * dr + 1]) >= 2))
    })
}

fn is_match_vertical(upper: Pattern, lower: Pattern) -> bool {
    upper[2] == lower[0] && upper[3] == lower[1]
}

fn is_match_horizontal(left: Pattern, right: Pattern) -> bool {
    left[1] == right[0] && left[3] == right[2]
}

fn is_match_diagonal(upper_left: Pattern, lower_right: Pattern) -> bool {
    upper_left[3] == lower_right[0]
}

#[derive(Debug, Clone, Copy)]
struct Node {
    upper_pattern: Pattern,
    left_pattern: Pattern,
    upper_left_pattern: Pattern,
    pattern: Pattern,
}
