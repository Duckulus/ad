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
    sieve[1] = false;


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

#[test]
pub fn eratos_test() {
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