/// Returns a Vec of booleans where a given value is true iff its index is a prime number
/// The Vec will be of size n+1
///
/// # Example
/// ```
/// use ad::numbers::eratos::eratos;
/// let sieve = eratos(10);
/// assert!(!sieve[4]);
/// assert!(sieve[7]);
/// ```
pub fn eratos(n: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; n + 1];
    sieve[0] = false;
    if n > 0 {
        sieve[1] = false;
    }


    let mut i = 2;
    while i * i < n {
        if sieve[i] {
            let mut k = i + i;
            while k <= n {
                sieve[k] = false;
                k += i;
            }
        }
        i += 1;
    }
    sieve
}

/// Returns a Vec containing the prime factors of u in ascending order
pub fn prime_factors(u: usize) -> Vec<i32> {
    if u == 1 {
        return vec![];
    }
    let mut prime_factors = Vec::new();
    let sieve = eratos(u / 2);
    let mut remainder = u;

    for i in 2..sieve.len() {
        if sieve[i] {
            while remainder % i == 0 {
                remainder = remainder / i;
                prime_factors.push(i as i32);
            }
        }
    }

    if prime_factors.is_empty() {
        prime_factors.push(u as i32);
    }

    prime_factors
}

#[test]
fn eratos_test() {
    let primes_below_100 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mut sieve = eratos(100);
    for p in primes_below_100 {
        assert!(sieve[p]);
        sieve[p] = false;
    }
    for b in sieve {
        assert!(!b);
    }
}

#[test]
fn prime_factors_test() {
    assert_eq!(prime_factors(1), vec![]);
    assert_eq!(prime_factors(2), vec![2]);
    assert_eq!(prime_factors(6), vec![2, 3]);
    assert_eq!(prime_factors(22), vec![2, 11]);
    assert_eq!(prime_factors(152), vec![2, 2, 2, 19]);
    assert_eq!(prime_factors(200), vec![2, 2, 2, 5, 5]);
}
