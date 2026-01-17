use std::{cmp, collections::BTreeSet};

use itertools::{Itertools, enumerate};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        ab: [(u64, u8); n],
        wxy: [(Usize1, u64, u8); q],
    }

    let mut sloppy1 = BTreeSet::<Horse>::new();
    let mut sloppy2 = BTreeSet::<Horse>::new();
    let mut polite1 = BTreeSet::<Horse>::new();
    let mut polite2 = BTreeSet::<Horse>::new();
    for (i, &(a, b)) in enumerate(&ab) {
        let horse = Horse::new(i, a, b);
        if b == 1 {
            sloppy1.insert(horse);
        } else {
            polite1.insert(horse);
        }
    }

    let mut score = ab.iter().map(|&(a, _)| a).sum::<u64>();

    while polite1.len() > sloppy2.len()
        && let Some(horse) = pop_unfortunate_horse(&mut sloppy1, &mut polite1)
    {
        if horse.tidiness == 1 {
            sloppy2.insert(horse);
        } else {
            polite2.insert(horse);
        }
        score += horse.mood;
    }

    let mut horses = enumerate(&ab)
        .map(|(i, &(a, b))| Horse::new(i, a, b))
        .collect_vec();
    for &(w, x, y) in &wxy {
        let prev_horse = horses[w];
        if prev_horse.tidiness == 1 {
            if sloppy1.remove(&prev_horse) {
                score -= prev_horse.mood;
            } else {
                sloppy2.remove(&prev_horse);
                score -= 2 * prev_horse.mood;
            }
        } else {
            if polite1.remove(&prev_horse) {
                score -= prev_horse.mood;
            } else {
                polite2.remove(&prev_horse);
                score -= 2 * prev_horse.mood;
            }
        }

        let new_horse = Horse::new(w, x, y);
        if new_horse.tidiness == 1 {
            sloppy1.insert(new_horse);
        } else {
            polite1.insert(new_horse);
        }
        score += new_horse.mood;
        horses[new_horse.id] = new_horse;

        for _ in 0..2 {
            if let Some(horse) = sloppy2.pop_first() {
                sloppy1.insert(horse);
                score -= horse.mood;
            }
            if let Some(horse) = polite2.pop_first() {
                polite1.insert(horse);
                score -= horse.mood;
            }
        }

        while polite1.len() > sloppy2.len()
            && let Some(horse) = pop_unfortunate_horse(&mut sloppy1, &mut polite1)
        {
            if horse.tidiness == 1 {
                sloppy2.insert(horse);
            } else {
                polite2.insert(horse);
            }
            score += horse.mood;
        }

        println!("{score}");
    }
}

fn pop_unfortunate_horse(
    sloppy1: &mut BTreeSet<Horse>,
    polite1: &mut BTreeSet<Horse>,
) -> Option<Horse> {
    if polite1.len() <= 1 {
        return sloppy1.pop_last();
    }

    match (sloppy1.last(), polite1.last()) {
        (None, None) => None,
        (None, Some(_)) => polite1.pop_last(),
        (Some(_), None) => sloppy1.pop_last(),
        (Some(sloppy_horse), Some(polite_horse)) => {
            if sloppy_horse.mood >= polite_horse.mood {
                sloppy1.pop_last()
            } else {
                polite1.pop_last()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Horse {
    id: usize,
    mood: u64,
    tidiness: u8,
}

impl PartialEq for Horse {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.mood == other.mood
    }
}

impl Eq for Horse {}

impl PartialOrd for Horse {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Horse {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.mood.cmp(&other.mood) {
            cmp::Ordering::Equal => self.id.cmp(&other.id),
            ord => ord,
        }
    }
}

impl Horse {
    fn new(id: usize, mood: u64, tidiness: u8) -> Self {
        Self { id, mood, tidiness }
    }
}
