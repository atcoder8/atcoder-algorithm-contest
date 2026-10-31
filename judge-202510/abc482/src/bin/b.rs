use proconio::{input, marker::Chars};

const N: usize = 5;

fn main() {
    input! {
        ss: [Chars; N],
    }

    let mut min_cost = N;
    for i in 0..N {
        let cost = (0..N).filter(|&j| ss[i][j] == '.').count();
        min_cost = min_cost.min(cost);
    }
    for j in 0..N {
        let cost = (0..N).filter(|&i| ss[i][j] == '.').count();
        min_cost = min_cost.min(cost);
    }
    {
        let cost = (0..N).filter(|&i| ss[i][i] == '.').count();
        min_cost = min_cost.min(cost);
    }
    {
        let cost = (0..N).filter(|&i| ss[i][4 - i] == '.').count();
        min_cost = min_cost.min(cost);
    }

    println!("{min_cost}");
}
