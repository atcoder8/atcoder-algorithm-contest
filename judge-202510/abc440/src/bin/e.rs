use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        mut aa: [i64; n],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));

    let mut heap = BinaryHeap::<Node>::from([Node {
        sum_score: aa[0] * k as i64,
        lowest_pos: 0,
        num_lowest: k,
        num_second_lowest: 0,
    }]);

    let mut ss = vec![];
    while let Some(node) = heap.pop()
        && ss.len() < x
    {
        let Node {
            sum_score,
            lowest_pos,
            num_lowest,
            num_second_lowest,
        } = node;

        ss.push(sum_score);

        if num_second_lowest > 0 {
            heap.push(Node {
                sum_score: sum_score - aa[lowest_pos - 1] + aa[lowest_pos],
                lowest_pos,
                num_lowest: num_lowest + 1,
                num_second_lowest: num_second_lowest - 1,
            });
        }

        if num_lowest > 0 && lowest_pos + 1 < n {
            heap.push(Node {
                sum_score: sum_score - aa[lowest_pos] + aa[lowest_pos + 1],
                lowest_pos: lowest_pos + 1,
                num_lowest: 1,
                num_second_lowest: num_lowest - 1,
            });
        }
    }

    println!("{}", ss.iter().join("\n"));
}

#[derive(Debug, Clone)]
struct Node {
    sum_score: i64,
    lowest_pos: usize,
    num_lowest: usize,
    num_second_lowest: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.sum_score == other.sum_score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum_score.cmp(&other.sum_score)
    }
}
