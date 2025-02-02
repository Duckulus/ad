use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

static RADIX: u32 = 10;

#[derive(Debug)]
pub enum BigIntegerError {
    ParseError,
}

/// Represents an immutable integer of arbitrary size
#[derive(Debug, Clone)]
pub struct BigInteger {
    digits: Vec<u8>,
    neg: bool,
}

static ZERO: BigInteger = BigInteger {
    digits: Vec::new(),
    neg: false,
};

impl BigInteger {
    /// returns the BigInteger represented by this string
    /// throws an error if the string does not represent a valid integer
    pub fn from_str(value: &str) -> Result<Self, BigIntegerError> {
        if value.len() == 0 {
            return Ok(ZERO.clone());
        }
        let chars: Vec<char> = value.chars().collect();
        let mut digits: Vec<u8> = Vec::with_capacity(chars.len());
        for i in (1..chars.len()).rev() {
            if !chars[i].is_digit(RADIX) {
                return Err(BigIntegerError::ParseError);
            }
            digits.push(chars[i].to_digit(RADIX).unwrap() as u8);
        }
        let mut neg = false;
        if chars[0] == '-' {
            neg = true;
        } else if chars[0].is_digit(RADIX) {
            digits.push(chars[0].to_digit(RADIX).unwrap() as u8)
        } else {
            return Err(BigIntegerError::ParseError);
        }

        Ok(Self { digits, neg })
    }

    pub fn from_i32(value: i32) -> Self {
        Self::from_str(value.to_string().as_str()).unwrap()
    }

    pub fn from_u64(value: u64) -> Self {
        Self::from_str(value.to_string().as_str()).unwrap()
    }

    /// returns a new BigInteger with the same magnitude and the specified sign
    pub fn with_sign(&self, neg: bool) -> Self {
        Self {
            digits: self.digits.clone(),
            neg,
        }
    }

    /// returns a new BigInteger with the same magnitude and the opposite sign
    pub fn negate(&self) -> Self {
        self.with_sign(!self.neg)
    }

    /// returns a new BigInteger with the same magnitude and a positive sign
    pub fn as_positive(&self) -> Self {
        self.with_sign(false)
    }

    /// returns a new BigInteger with the same magnitude and a negative sign
    pub fn as_negative(&self) -> Self {
        self.with_sign(true)
    }

    pub fn is_negative(&self) -> bool {
        self.digits.len() > 0 && self.neg
    }

    pub fn is_zero(&self) -> bool {
        self.digits.len() == 0
    }

    /// returns true if this BigInteger represents a larger number than other
    pub fn greater_than(&self, other: &Self) -> bool {
        if self.is_negative() && other.is_negative() {
            return other.as_positive().greater_than(&self.as_positive());
        } else if self.is_negative() && !other.is_negative() {
            return false;
        } else if other.is_negative() && !self.is_negative() {
            return true;
        }
        if self.digits.len() > other.digits.len() {
            return true;
        } else if other.digits.len() > self.digits.len() {
            return false;
        }
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] > other.digits[i] {
                return true;
            } else if other.digits[i] > self.digits[i] {
                return false;
            }
        }
        false
    }

    /// returns true if this BigInteger represents a larger number than other
    pub fn equals(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return false;
        }
        if self.digits.len() != 0 && self.neg != other.neg {
            return false;
        }
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] != other.digits[i] {
                return false;
            }
        }
        true
    }

    /// shifts the decimal representation of this BigInteger to the left by n digits
    /// the effect is the same as multiplying it by 10^n
    pub fn shift_left(&self, n: usize) -> Self {
        let mut digits = vec![0; n];
        digits.extend(self.digits.clone());
        Self {
            digits,
            neg: self.neg,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        if self.is_negative() && other.is_negative() {
            return self.as_positive().add(&other.as_positive()).as_negative();
        } else if self.is_negative() && !other.is_negative() {
            return other.sub(&self.as_positive());
        } else if other.is_negative() && !self.is_negative() {
            return self.sub(&other.as_positive());
        }

        let mut a = self;
        let mut b = other;
        if b.greater_than(a) {
            std::mem::swap(&mut a, &mut b);
        }

        let mut carry = 0;
        let mut sum_digits = Vec::with_capacity(a.digits.iter().len());
        for i in 0..a.digits.len() {
            let mut sum = a.digits[i];
            if i < b.digits.iter().len() {
                sum += b.digits[i];
            }
            sum += carry;
            carry = 0;
            if sum > 9 {
                carry = sum / 10;
                sum = sum % 10;
            }
            sum_digits.push(sum);
        }
        if carry > 0 {
            sum_digits.push(carry);
        }

        Self {
            digits: sum_digits,
            neg: false,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        if self.is_negative() && other.is_negative() {
            return self.add(&other.as_positive());
        } else if self.is_negative() && !other.is_negative() {
            return self.as_positive().add(other).as_negative();
        } else if !self.is_negative() && other.is_negative() {
            return self.add(&other.as_positive());
        }

        let mut a = self;
        let mut b = other;
        let mut neg = false;
        if b.greater_than(a) {
            std::mem::swap(&mut a, &mut b);
            neg = true;
        }

        let mut carry = 0;
        let mut diff_digits = Vec::with_capacity(a.digits.iter().len());
        for i in 0..a.digits.len() {
            let mut ad = a.digits[i];
            let mut bd = 0;
            if i < b.digits.iter().len() {
                bd += b.digits[i];
            }
            bd += carry;
            carry = 0;

            while ad < bd {
                ad += 10;
                carry += 1;
            }

            let diff = ad - bd;
            diff_digits.push(diff);
        }
        if carry > 0 {
            diff_digits.push(carry);
        }

        while !diff_digits.is_empty() && diff_digits[diff_digits.len() - 1] == 0 {
            diff_digits.pop();
        }

        Self {
            digits: diff_digits,
            neg,
        }
    }

    /// multiplies 2 BigIntegers by using Karatsuba Multiplication
    pub fn mul(&self, other: &BigInteger) -> Self {
        if self.neg != other.neg {
            return self.as_positive().mul(&other.as_positive()).as_negative();
        }
        let a = self.as_positive();
        let b = other.as_positive();
        if a.is_zero() || b.is_zero() {
            return ZERO.clone();
        }
        if a.digits.len() == 1 && b.digits.len() == 1 {
            return Self::from_u64(a.digits[0] as u64 * b.digits[0] as u64);
        }

        let a_digits = a.digits.clone();
        let a_len = a_digits.len();
        let b_digits = b.digits.clone();
        let b_len = b_digits.len();

        let len = max(a.digits.len(), b.digits.len());
        let half_len = len / 2;

        let ah_digits = a_digits[min(half_len, a_len)..min(len, a_len)].to_vec();
        let ah = Self {
            digits: ah_digits,
            neg: false,
        };
        let al_digits = a_digits[0..min(a_len, half_len)].to_vec();
        let al = Self {
            digits: al_digits,
            neg: false,
        };
        let bh_digits = b_digits[min(half_len, b_len)..min(len, b_len)].to_vec();
        let bh = Self {
            digits: bh_digits,
            neg: false,
        };
        let bl_digits = b_digits[0..min(b_len, half_len)].to_vec();
        let bl = Self {
            digits: bl_digits,
            neg: false,
        };

        let h = ah.mul(&bh);
        let l = al.mul(&bl);
        let m = ah.sub(&al).mul(&bl.sub(&bh));

        let result = h
            .shift_left(2 * half_len)
            .add(&m.add(&h).add(&l).shift_left(half_len))
            .add(&l);

        let mut d = result.digits.clone();
        let mut i = d.len() - 1;
        while i > 0 && d[i] == 0 {
            d.pop();
            i-=1;
        }

        Self {
            digits: d,
            neg: false
        }
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.neg { "-" } else { "" },
            self.digits
                .iter()
                .rev()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[test]
pub fn add_test() {
    let a = BigInteger::from_str("42").unwrap();
    let b = BigInteger::from_str("112").unwrap();
    let c = BigInteger::from_str("154").unwrap();
    assert!(a.add(&b).equals(&c));

    let a = BigInteger::from_str("13").unwrap();
    let b = BigInteger::from_str("-6").unwrap();
    let c = BigInteger::from_str("7").unwrap();
    assert!(a.add(&b).equals(&c));

    let a = BigInteger::from_str("-6").unwrap();
    let b = BigInteger::from_str("13").unwrap();
    let c = BigInteger::from_str("7").unwrap();
    assert!(a.add(&b).equals(&c));

    let a = BigInteger::from_str("-326").unwrap();
    let b = BigInteger::from_str("-4830").unwrap();
    let c = BigInteger::from_str("-5156").unwrap();
    assert!(a.add(&b).equals(&c))
}

#[test]
pub fn sub_test() {
    let a = BigInteger::from_str("612").unwrap();
    let b = BigInteger::from_str("83").unwrap();
    let c = BigInteger::from_str("529").unwrap();
    assert!(a.sub(&b).equals(&c));

    let a = BigInteger::from_str("83").unwrap();
    let b = BigInteger::from_str("612").unwrap();
    let c = BigInteger::from_str("-529").unwrap();
    assert!(a.sub(&b).equals(&c));

    let a = BigInteger::from_str("612").unwrap();
    let b = BigInteger::from_str("-83").unwrap();
    let c = BigInteger::from_str("695").unwrap();
    assert!(a.sub(&b).equals(&c));

    let a = BigInteger::from_str("-612").unwrap();
    let b = BigInteger::from_str("83").unwrap();
    let c = BigInteger::from_str("-695").unwrap();
    assert!(a.sub(&b).equals(&c));

    let a = BigInteger::from_str("-612").unwrap();
    let b = BigInteger::from_str("-83").unwrap();
    let c = BigInteger::from_str("-529").unwrap();
    assert!(a.sub(&b).equals(&c));

    let a = BigInteger::from_str("-83").unwrap();
    let b = BigInteger::from_str("-612").unwrap();
    let c = BigInteger::from_str("529").unwrap();
    assert!(a.sub(&b).equals(&c));
}

#[test]
pub fn mul_test() {
    let a = BigInteger::from_str("4725").unwrap();
    let b = BigInteger::from_str("9393").unwrap();
    let c = BigInteger::from_i32(4725 * 9393);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("4725").unwrap();
    let b = BigInteger::from_str("939").unwrap();
    let c = BigInteger::from_i32(4725 * 939);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("472").unwrap();
    let b = BigInteger::from_str("9").unwrap();
    let c = BigInteger::from_i32(472 * 9);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("472").unwrap();
    let b = BigInteger::from_str("-9").unwrap();
    let c = BigInteger::from_i32(472 * -9);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("472").unwrap();
    let b = BigInteger::from_str("-9").unwrap();
    let c = BigInteger::from_i32(472 * -9);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("-472").unwrap();
    let b = BigInteger::from_str("9").unwrap();
    let c = BigInteger::from_i32(-472 * 9);
    assert!(a.mul(&b).equals(&c));

    let a = BigInteger::from_str("-472").unwrap();
    let b = BigInteger::from_str("-9").unwrap();
    let c = BigInteger::from_i32(-472 * -9);
    assert!(a.mul(&b).equals(&c));
}
