use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut pp: [Usize1; n],
    }

    let queries = (0..q).map(|_| Query::read());

    let mut inv_pp = vec![0; n];
    for (i, &p) in enumerate(&pp) {
        inv_pp[p] = i;
    }

    for query in queries {
        match query {
            Query::Swap { x, y } => {
                let px = pp[x];
                let py = pp[y];
                pp.swap(x, y);
                inv_pp.swap(px, py);
            }
            Query::Replace => std::mem::swap(&mut pp, &mut inv_pp),
        }
    }

    let output = pp.iter().map(|p| p + 1).join(" ");
    println!("{output}");
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Swap { x: usize, y: usize },
    Replace,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                (x, y): (Usize1, Usize1),
            }

            Query::Swap { x, y }
        } else {
            Query::Replace
        }
    }
}
