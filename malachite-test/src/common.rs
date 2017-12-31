use malachite_native as native;
use malachite_gmp as gmp;
use num;
use rugint;
use std::str::FromStr;

pub fn gmp_natural_to_native(n: &gmp::natural::Natural) -> native::natural::Natural {
    native::natural::Natural::from_limbs_le(n.to_limbs_le().as_slice())
}

pub fn native_natural_to_gmp(n: &native::natural::Natural) -> gmp::natural::Natural {
    gmp::natural::Natural::from_limbs_le(n.to_limbs_le().as_slice())
}

pub fn num_biguint_to_native_natural(n: &num::BigUint) -> native::natural::Natural {
    native::natural::Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_natural_to_num_biguint(n: &native::natural::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_natural_to_num_biguint(n: &gmp::natural::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rugint_integer_to_native_natural(n: &rugint::Integer) -> native::natural::Natural {
    native::natural::Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_natural_to_rugint_integer(n: &native::natural::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_natural_to_rugint_integer(n: &gmp::natural::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_integer_to_native(n: &gmp::integer::Integer) -> native::integer::Integer {
    let (sign, limbs) = n.sign_and_limbs_le();
    native::integer::Integer::from_sign_and_limbs_le(sign, &limbs[..])
}

pub fn native_integer_to_gmp(n: &native::integer::Integer) -> gmp::integer::Integer {
    let (sign, limbs) = n.sign_and_limbs_le();
    gmp::integer::Integer::from_sign_and_limbs_le(sign, &limbs[..])
}

pub fn num_bigint_to_native_integer(n: &num::BigInt) -> native::integer::Integer {
    native::integer::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_integer_to_num_bigint(n: &native::integer::Integer) -> num::BigInt {
    num::BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_integer_to_num_bigint(n: &gmp::integer::Integer) -> num::BigInt {
    num::BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rugint_integer_to_native(n: &rugint::Integer) -> native::integer::Integer {
    native::integer::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_integer_to_rugint(n: &native::integer::Integer) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_integer_to_rugint(n: &gmp::integer::Integer) -> rugint::Integer {
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
