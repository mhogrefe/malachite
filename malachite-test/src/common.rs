use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use rugint;
use std::str::FromStr;

pub fn to_native(n: &gmp::Natural) -> native::Natural {
    let mut native = native::Natural::new();
    native.assign_limbs_le(n.limbs_le().as_slice());
    native
}

pub fn from_native(n: &native::Natural) -> gmp::Natural {
    let mut gmp = gmp::Natural::new();
    gmp.assign_limbs_le(n.limbs_le().as_slice());
    gmp
}

pub fn to_num(n: &gmp::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn from_num(n: &num::BigUint) -> gmp::Natural {
    gmp::Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn to_rugint(n: &gmp::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn from_rugint(n: &rugint::Integer) -> gmp::Natural {
    gmp::Natural::from_str(n.to_string().as_ref()).unwrap()
}
