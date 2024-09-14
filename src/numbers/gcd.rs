use crate::numbers::eratos::prime_factors;

/// Returns the greatest common divisor of u and v
/// The gcd is computed by multiplying the common prime factors of u and v
pub fn gcd_using_prime_factors(u: usize, v: usize) -> i32{
    let mut gcd = 1;
    let u_prime_factors = prime_factors(u);
    let v_prime_factors = prime_factors(v);

    let mut up = 0;
    let mut vp = 0;
    while up < u_prime_factors.len() && vp < v_prime_factors.len() {
        let factor_u = u_prime_factors[up];
        let factor_v = v_prime_factors[vp];
        if factor_u < factor_v {
            up += 1;
        } else if factor_u > factor_v {
            vp += 1;
        } else {
            gcd *= factor_u;
            up += 1;
            vp += 1;
        }
    }
    gcd
}

/// Returns the greatest common divisor of u and v
/// The gcd is computed using euclid's algorithm
pub fn gcd_euclid(u: usize, v: usize) -> i32 {
    let mut u = u;
    let mut v = v;
    while u > 0 {
        if u < v {
            let temp = u;
            u = v;
            v = temp;
        }
        u = u % v;
    }
    v as i32
}

#[test]
pub fn gcd_using_prime_factors_test() {
    assert_eq!(gcd_using_prime_factors(8, 12), 4);
    assert_eq!(gcd_using_prime_factors(84, 231), 21);
    assert_eq!(gcd_using_prime_factors(233, 144), 1);
}

#[test]
pub fn gcd_euclid_test() {
    assert_eq!(gcd_euclid(8, 12), 4);
    assert_eq!(gcd_euclid(84, 231), 21);
    assert_eq!(gcd_euclid(233, 144), 1);
}