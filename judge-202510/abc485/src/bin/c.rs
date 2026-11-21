use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        cc: [u32; n],
    }

    let mut distances = vec![n; n];
    let mut prev_cnt = None::<usize>;
    let mut curr_pos = 0;
    for (cnt, _) in cc.iter().dedup_with_count() {
        if let Some(prev_cnt) = prev_cnt {
            for i in 0..prev_cnt {
                let prev_dist = &mut distances[curr_pos - prev_cnt + i];
                *prev_dist = (*prev_dist).min(prev_cnt - i);
            }
            for i in 0..cnt {
                let dist = &mut distances[curr_pos + i];
                *dist = (*dist).min(i + 1);
            }
        }
        prev_cnt = Some(cnt);
        curr_pos += cnt;
    }

    let output = distances.iter().join(" ");
    println!("{output}");
}
