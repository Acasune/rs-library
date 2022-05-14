fn eratosthenes(size: usize) -> Vec<bool> {
    let mut is_primes = vec![true; size + 1];
    is_primes[0] = false;
    is_primes[1] = false;
    for i in 2..=size {
        if is_primes[i] {
            let mut j = 2 * i;
            while j <= size {
                is_primes[j] = false;
                j += i;
            }
        }
    }
    is_primes
}

fn get_primes(size: usize) -> Vec<i64> {
    let mut is_primes = vec![true; size + 1];
    let mut primes = vec![];
    is_primes[0] = false;
    is_primes[1] = false;
    for i in 2..=size {
        if is_primes[i] {
            primes.push(i as i64);
            let mut j = 2 * i;
            while j <= size {
                is_primes[j] = false;
                j += i;
            }
        }
    }
    primes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eratosthenes() {
        use std::collections::HashSet;
        let primes: HashSet<usize> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
        .into_iter()
        .collect();
        let is_primes = eratosthenes(100);
        for i in 0..=100 {
            if primes.contains(&i) {
                assert!(is_primes[i]);
            } else {
                assert!(!is_primes[i]);
            }
        }
    }

    #[test]
    fn test_get_primes() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        let actual = get_primes(100);
        assert_eq!(expected, actual);
    }
}
