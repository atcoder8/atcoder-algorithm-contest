// unfinished

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        n: usize,
        s: Chars,
        rr: [u64; n],
    }

    let max_r = *rr.iter().max().unwrap();
    let positions_max = rr.iter().positions(|&r| r == max_r).collect_vec();

    let groups = s.iter().cloned().dedup_with_count().collect_vec();
    let mut left = 0;
    let mut vacant_sections = vec![];
    let mut tree_sections = vec![];
    for &(cnt, ch) in &groups {
        let right = left + cnt;

        if ch == '.' {
            vacant_sections.push((left, right));
        } else {
            tree_sections.push((left, right));
        }

        left = right;
    }

    if positions_max.iter().any(|&pos| s[pos] == '.') {
        return (0..vacant_sections.len())
            .map(|section_idx| {
                let (curr_left, curr_right) = vacant_sections[section_idx];
                let curr_rr = &rr[curr_left..curr_right];
                let max_r_count = curr_rr.iter().filter(|&&r| r == max_r).count();

                let mut adjacent_section_indexes = vec![];
                if section_idx > 0 {
                    adjacent_section_indexes.push(section_idx - 1);
                }
                if section_idx + 1 < vacant_sections.len() {
                    adjacent_section_indexes.push(section_idx + 1);
                }

                max_r_count
                    * adjacent_section_indexes
                        .iter()
                        .map(|&adj_idx| {
                            let (adj_left, adj_right) = vacant_sections[adj_idx];
                            count_max_element(&rr[adj_left..adj_right])
                        })
                        .sum::<usize>()
            })
            .sum();
    }

    todo!()
}

fn count_max_element<T: Ord>(seq: &[T]) -> usize {
    let Some(max) = seq.iter().max() else {
        return 0;
    };

    seq.iter().filter(|&elem| elem == max).count()
}
