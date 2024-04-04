use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bocchi_shut_up(flag: i32, seq: &[i32], size: i32) -> i32 {
    let frequency = seq.iter()
        .filter(|&x| {
            *x / 10 == flag
        })
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });

    let max_freq = frequency.values().max().copied().unwrap_or(0);
    let most_freq_num = frequency.iter().filter(|&(_, freq)| {
        *freq == max_freq
    }).map(|(num, _)| *num).collect::<Vec<_>>();

    if most_freq_num.len() > 1 {
        10
    } else {
        most_freq_num[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bocchi_shut_up_test_when_10() {
        assert_eq!(bocchi_shut_up(1, &[11, 12, 13], 3), 10);
        assert_eq!(bocchi_shut_up(2, &[21, 22, 23], 3), 10);
    }

    #[test]
    fn bocchi_shut_up_test_when_not_10() {
        assert_eq!(bocchi_shut_up(1, &[11, 12, 12], 3), 12);
        assert_eq!(bocchi_shut_up(2, &[21, 21, 22], 3), 21);
    }
}