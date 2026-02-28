use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut score = 0_usize;
    let mut num_a = 0_usize;
    let mut num_c = s.iter().filter(|&&ch| ch == 'C').count();
    let mut consumed_c = 0_usize;
    for &ch in &s {
        match ch {
            'A' => num_a += 1,
            'B' => {
                if num_a > 0 && num_c > 0 {
                    score += 1;
                    num_a -= 1;
                    num_c -= 1;
                    consumed_c += 1;
                }
            }
            'C' => {
                if consumed_c > 0 {
                    consumed_c -= 1;
                } else {
                    num_c -= 1;
                }
            }
            _ => panic!(),
        }
    }

    println!("{score}");
}
