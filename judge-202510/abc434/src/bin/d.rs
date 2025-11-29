use argio::proconio::{input, marker::Usize1};
use itertools::Itertools;
use ndarray::prelude::*;

const SKY_SIZE: usize = 2000;

fn main() {
    input! {
        n: usize,
        udlr: [(Usize1, usize, Usize1, usize); n],
    }

    let mut imos = Array::from_elem([SKY_SIZE + 1; 2], 0_i64);
    for &(u, d, l, r) in &udlr {
        imos[(u, l)] += 1;
        imos[(u, r)] -= 1;
        imos[(d, l)] -= 1;
        imos[(d, r)] += 1;
    }

    for row in 0..=SKY_SIZE {
        for col in 0..SKY_SIZE {
            imos[(row, col + 1)] += imos[(row, col)];
        }
    }
    for col in 0..=SKY_SIZE {
        for row in 0..SKY_SIZE {
            imos[(row + 1, col)] += imos[(row, col)];
        }
    }

    let num_zeros = imos
        .slice(s![0..SKY_SIZE, 0..SKY_SIZE])
        .iter()
        .filter(|&&cnt| cnt == 0)
        .count();

    let mut acc = Array2::from_shape_fn((SKY_SIZE + 1, SKY_SIZE + 1), |(row, col)| {
        (row > 0 && col > 0 && imos[(row - 1, col - 1)] == 1) as usize
    });
    for row in 0..=SKY_SIZE {
        for col in 0..SKY_SIZE {
            acc[(row, col + 1)] += acc[(row, col)];
        }
    }
    for col in 0..=SKY_SIZE {
        for row in 0..SKY_SIZE {
            acc[(row + 1, col)] += acc[(row, col)];
        }
    }

    let solve = |cloud_id: usize| {
        let (u, d, l, r) = udlr[cloud_id];
        num_zeros + (acc[(u, l)] + acc[(d, r)]) - (acc[(u, r)] + acc[(d, l)])
    };

    let output = (0..n).map(solve).join("\n");
    println!("{output}");
}
