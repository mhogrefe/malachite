use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::{BigInt, BigUint};
use rug;
use std::str::FromStr;

pub fn biguint_to_natural(n: &BigUint) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rug_integer_to_natural(n: &rug::Integer) -> Natural {
    Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn bigint_to_integer(n: &BigInt) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_bigint(n: &Integer) -> BigInt {
    BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rug_integer_to_integer(n: &rug::Integer) -> Integer {
    Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn integer_to_rug_integer(n: &Integer) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
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
