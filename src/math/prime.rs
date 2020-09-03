use std::collections::HashMap;

/// Sieve of Eratosthenes
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut tbl = vec![true; n + 1];
    tbl[0] = false;
    tbl[1] = false;
    for p in 2..n {
        if tbl[p] {
            for q in (2 * p..).step_by(p).take_while(|&q| q <= n) {
                tbl[q] = false;
            }
        }
    }
    tbl
}

/// Find n or less primes using Sieve of Eratosthenes
pub fn primes(n: usize) -> Vec<usize> {
    let tbl = sieve_of_eratosthenes(n);
    let primes = tbl
        .into_iter()
        .enumerate()
        .filter(|&(_, is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    primes
}

/// Prime factorization
pub fn factors_trial_division(mut n: usize) -> Vec<(usize, usize)> {
    let ps = primes(n);
    let mut factors = vec![];
    for p in ps.into_iter() {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        if count > 0 {
            factors.push((p, count));
        }
    }
    factors
}

/// Fast prime factorization (called 'osa_k method')
/// O(n loglog n)
pub fn factors_osa_k(n: usize) -> HashMap<usize, usize> {
    // Compute the smallest prime factor for (0..=n)
    let mut tbl = (0..=n).collect::<Vec<usize>>();
    for p in 2.. {
        if p * p > n {
            break;
        }
        if tbl[p] == p {
            for mul in (2 * p..=n).step_by(p) {
                if tbl[mul] == mul {
                    tbl[mul] = p;
                }
            }
        }
    }

    let mut factors = HashMap::new();
    let mut t = n;
    while t > 1 {
        factors.entry(tbl[t]).and_modify(|e| *e += 1).or_insert(1);
        t /= tbl[t];
    }
    factors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sieve_of_eratosthenes_test() {
        assert_eq!(
            sieve_of_eratosthenes(10),
            vec![false, false, true, true, false, true, false, true, false, false, false]
        );
    }

    #[test]
    fn primes_test() {
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn factors_trial_division_test() {
        assert_eq!(factors_trial_division(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn factors_osa_k_test() {
        let mut fac = HashMap::new();
        fac.insert(2, 2);
        fac.insert(3, 1);
        fac.insert(5, 1);
        assert_eq!(factors_osa_k(60), fac);
    }
}
