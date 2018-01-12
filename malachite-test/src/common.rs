use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::{BigInt, BigUint};
use rugint;
use std::str::FromStr;

pub fn biguint_to_natural(n: &BigUint) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rugint_integer_to_natural(n: &rugint::Integer) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_rugint_integer(n: &Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn bigint_to_integer(n: &BigInt) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_bigint(n: &Integer) -> BigInt {
    BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rugint_integer_to_integer(n: &rugint::Integer) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_rugint_integer(n: &Integer) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GenerationMode {
    Exhaustive,
    Random(u32),
}

impl GenerationMode {
    pub fn name(&self) -> &str {
        match *self {
            GenerationMode::Exhaustive => "exhaustive",
            GenerationMode::Random(_) => "random",
        }
    }
}
