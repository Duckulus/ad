static RADIX: u32 = 10;

#[derive(Debug)]
pub enum BigIntegerError {
    ParseError,
}

/// Represents an integer of arbitrary size
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

    pub fn with_sign(&self, neg: bool) -> Self {
        Self {
            digits: self.digits.clone(),
            neg,
        }
    }

    pub fn negate(&self) -> Self {
        self.with_sign(!self.neg)
    }

    pub fn as_positive(&self) -> Self {
        self.with_sign(false)
    }

    pub fn as_negative(&self) -> Self {
        self.with_sign(true)
    }

    pub fn greater_than(&self, other: &Self) -> bool {
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

    pub fn equals(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return false;
        }
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] != other.digits[i] {
                return false;
            }
        }
        true
    }

    pub fn add(&self, other: &Self) -> Self {
        if self.neg && other.neg {
            return self.as_positive().add(&other.as_positive()).as_negative();
        } else if self.neg && !other.neg {
            return other.sub(&self.as_positive());
        } else if other.neg && !self.neg {
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
        if self.neg && other.neg {
            return self.add(&other.as_positive());
        } else if self.neg && !other.neg {
            return self.as_positive().add(other).as_negative();
        } else if !self.neg && other.neg {
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

        while !diff_digits.is_empty() && diff_digits[diff_digits.len()-1] == 0 {
            diff_digits.pop();
        }

        Self {
            digits: diff_digits,
            neg,
        }
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
