use std::{cmp::max, ops::{Add, Div, Mul, Sub}, sync::Mutex};

use lazy_static::lazy_static;

// represents an integer by its prime factors
#[derive(Debug)]
pub struct FactoredNumber {
    factors: Vec<i64>
}

lazy_static! {
    static ref PRIME_NUMBERS: Mutex<Vec<u64>> = Mutex::new(vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    ]);
}


impl FactoredNumber {
    pub fn new() -> FactoredNumber {
        FactoredNumber {
            factors: Vec::new(),
        }
    }

    pub fn get_prime_number(index: u64) -> u64 {
        let mut locked_primes = PRIME_NUMBERS.lock().unwrap();
        if index < locked_primes.len().try_into().unwrap() {
            locked_primes[index as usize]
        } else {
            let mut n = locked_primes.len() as u64;
            let mut p = locked_primes[n as usize - 1];
            while n < index {
                p += 2;
                let mut is_prime = true;
                for i in 0..n {
                    if p % locked_primes[i as usize] == 0 {
                        is_prime = false;
                        break;
                    }
                    if locked_primes[i as usize] * locked_primes[i as usize] > p {
                        break;
                    }
                }
                if is_prime {
                    locked_primes.push(p);
                    n += 1;
                }
            }
            p
        }
    }

    pub fn from_number(mut n: u64) -> FactoredNumber {
        let mut factors = Vec::new();
        let mut i = 0;
        while n != 1 {
            let p = FactoredNumber::get_prime_number(i);
            let mut count = 0; 
            while n % p == 0 {
                count += 1;
                n /= p;
            }
            factors.push(count);
            i += 1;
        }
        FactoredNumber {
            factors,
        }
    }
    pub fn to_number(&self) -> u64 {
        let mut n = 1;
        for i in 0..self.factors.len() {
            n *= FactoredNumber::get_prime_number(i as u64).pow(self.factors[i] as u32);
        }
        n
    }
}

//multiplication for FactoredNumber
//just add the factors
impl Mul for FactoredNumber {
    type Output = FactoredNumber;

    fn mul(self, other: FactoredNumber) -> FactoredNumber {
        let mut factors = Vec::new();
        for i in 0..max(self.factors.len(), other.factors.len()) {
            factors.push(
                if i < self.factors.len() {
                    self.factors[i]
                } else {
                    0
                } + if i < other.factors.len() {
                    other.factors[i]
                } else {
                    0
                }
            );
        }
        FactoredNumber {
            factors,
        }
    }
}

//division for FactoredNumber
//just subtract the factors
impl Div for FactoredNumber {
    type Output = FactoredNumber;

    fn div(self, other: FactoredNumber) -> FactoredNumber {
        let mut factors = Vec::new();
        for i in 0..max(self.factors.len(), other.factors.len()) {
            factors.push(
                if i < self.factors.len() {
                    self.factors[i]
                } else {
                    0
                } - if i < other.factors.len() {
                    other.factors[i]
                } else {
                    0
                }
            );
        }
        FactoredNumber {
            factors,
        }
    }
}

impl Add for FactoredNumber {
    type Output = FactoredNumber;

    fn add(self, other: FactoredNumber) -> FactoredNumber {
        FactoredNumber::from_number(self.to_number() + other.to_number())
    }
}

impl Sub for FactoredNumber {
    type Output = FactoredNumber;

    fn sub(self, other: FactoredNumber) -> FactoredNumber {
        FactoredNumber::from_number(self.to_number() - other.to_number())
    }
}