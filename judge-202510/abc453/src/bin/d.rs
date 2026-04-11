use itertools::{Itertools, iproduct};
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    match solve() {
        Some(ans) => println!("Yes\n{}", ans),
        None => println!("No"),
    }
}

fn solve() -> Option<String> {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let find_coord = |ch: char| {
        iproduct!(0..h, 0..w)
            .find(|&(row, col)| sss[row][col] == ch)
            .unwrap()
    };

    let start_coord = find_coord('S');
    let goal_coord = find_coord('G');

    let mut route = String::new();
    let mut cost_array = vec![vec![[None::<usize>; 4]; w]; h];
    let mut stack = (0..4).map(|dir| (start_coord, dir, 0, true)).collect_vec();
    while let Some(((row, col), dir, cand_cost, forward)) = stack.pop() {
        if !forward {
            route.pop();
            continue;
        }

        route.push(['U', 'L', 'R', 'D'][dir]);

        if (row, col) == goal_coord {
            return Some(route[1..].to_string());
        }

        let s = sss[row][col];
        let cost = &mut cost_array[row][col][dir];

        if s == '#' || cost.is_some() {
            continue;
        }

        *cost = Some(cand_cost);

        let possible_dirs = match s {
            'o' => vec![dir],
            'x' => (0..4).filter(|&next_dir| next_dir != dir).collect_vec(),
            _ => vec![0, 1, 2, 3],
        };

        stack.extend(possible_dirs.iter().flat_map(|&next_dir| {
            let (dr, dc) = DIFFS[next_dir];
            let adj_row = row.wrapping_add(dr);
            let adj_col = col.wrapping_add(dc);
            if adj_row < h && adj_col < w {
                vec![
                    ((row, col), dir, cand_cost, false),
                    ((adj_row, adj_col), next_dir, cand_cost + 1, true),
                ]
            } else {
                vec![]
            }
        }));
    }

    None
}
