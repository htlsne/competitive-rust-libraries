#![allow(unused)]
use std::collections::{HashSet, HashMap};

pub fn gcd(a: usize, b: usize) -> usize {
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn prime_sieve(n: usize) -> Vec<u64> {
    let mut table: Vec<u64> = vec![0; n + 1];
    let mut primes: Vec<u64> = Vec::new();

    for i in 2..=n {
        if table[i] == 0 {
            primes.push(i as u64);
            for j in 2..n {
                if i * j > n {
                    break
                }
                table[i * j] = 1
            }
        }
    }

    primes
}

pub fn prime_factor(n: u64) -> HashMap<u64, u64> {
    let sqrt = (n as f64).sqrt() as u64;
    let mut rest = n;
    let mut factors = HashMap::new();
    for d in 2..=sqrt {
        while rest % d == 0 {
            let count = factors.entry(d).or_insert(0);
            *count += 1;
            rest /= d;
        }
    }
    if rest != 1 {
        factors.insert(rest, 1);
    }

    factors
}

pub fn divisors_vec(n: u64) -> Vec<u64> {
    prime_factor(n).iter().fold(vec![1], |acc, (&p, &pow)| {
        (0..=pow).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

pub fn divisors_set(n: u64) -> HashSet<u64> {
    let mut set = HashSet::new();
    set.insert(1);

    prime_factor(n).iter().fold(set, |acc, (&p, &pow)| {
        (0..=pow).flat_map(|i| acc.iter().map(move |a| a * p.pow(i as u32))).collect()
    })
}

pub fn binom_coef(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    if n - k < k {
        return binom_coef(n, n - k);
    }

    let mut ret = 1;
    for i in 1..=k {
        ret = ret * (n - k + i) / i
    }
    ret
}

pub struct Factorize {
    min_factor: Vec<usize>,
}

impl Factorize {
    pub fn new(max: usize) -> Factorize {
        let min_factor = Factorize::prime_min_factor(max);
        Factorize { min_factor }
    }

    fn prime_min_factor(max: usize) -> Vec<usize> {
        let mut min_factor = vec![0usize; max + 1];
        min_factor[0] = 0;
        min_factor[1] = 1;

        for i in 2..=max {
            if min_factor[i] == 0 {
                min_factor[i] = i;
                let mut j = i * 2;
                while j <= max {
                    if min_factor[j] == 0 {
                        min_factor[j] = i;
                    }
                    j += i;
                }
            }
        }

        min_factor
    }

    pub fn factorize(&mut self, n: usize) -> Vec<(usize, usize)> {
        let mut factors = vec![];
        let mut n = n;
        while n != 1 {
            let prime = self.min_factor[n];
            let mut exp = 0;
            while self.min_factor[n] == prime {
                exp += 1;
                n /= prime;
            }
            factors.push((prime, exp));
        }

        factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binom_coef_test(){
        assert_eq!(6, binom_coef(4, 2));
        assert_eq!(3, binom_coef(3, 2));
        assert_eq!(20, binom_coef(6, 3));
        assert_eq!(1, binom_coef(10, 0));
        assert_eq!(10, binom_coef(10, 1));
        assert_eq!(1, binom_coef(10, 10));
        assert_eq!(0, binom_coef(2, 3));
    }
}
