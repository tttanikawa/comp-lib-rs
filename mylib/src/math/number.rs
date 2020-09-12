/// Greatest Common Divisor
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least Common Multiplier
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Enumerate divisors
pub fn divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in (1..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(5, 10), 5);
        assert_eq!(gcd(12, 16), 4);
        assert_eq!(gcd(111, 185), 37);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 10), 10);
        assert_eq!(lcm(12, 16), 48);
        assert_eq!(lcm(111, 185), 555);
    }

    #[test]
    fn divisors_test() {
        let mut divs = divisors(4);
        divs.sort();
        assert_eq!(divs, vec![1, 2, 4]);

        let mut divs = divisors(12);
        divs.sort();
        assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);

        let mut divs = divisors(25);
        divs.sort();
        assert_eq!(divs, vec![1, 5, 25]);

        let mut divs = divisors(31);
        divs.sort();
        assert_eq!(divs, vec![1, 31]);
    }
}
