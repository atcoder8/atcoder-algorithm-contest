use itertools::{Itertools, chain, enumerate};
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let queries = (0..q).map(|_| Query::read());

    let mut toggle_times_by_square = vec![vec![]; n];
    let mut paint_events = vec![];
    for (i, query) in enumerate(queries) {
        match query {
            Query::Toggle(x) => toggle_times_by_square[x].push(i),
            Query::Paint(c) => paint_events.push((i, c)),
        }
    }

    let solve = |i: usize| {
        let times = chain!([0], toggle_times_by_square[i].iter().copied(), [q]);
        let sections: Vec<(usize, usize)> = times.tuples().collect();
        for (left, right) in sections.into_iter().rev() {
            let upper_bound = paint_events.upper_bound_by_key(&right, |v| v.0);
            if upper_bound > 0 && paint_events[upper_bound - 1].0 >= left {
                return paint_events[upper_bound - 1].1;
            }
        }

        'a'
    };

    let output = (0..n).map(solve).collect::<String>();
    println!("{output}");
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Toggle(usize),
    Paint(char),
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                x: Usize1,
            }

            Query::Toggle(x)
        } else {
            input! {
                c: char,
            }

            Query::Paint(c)
        }
    }
}
