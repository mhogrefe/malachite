use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use rugint;
use std::str::FromStr;

pub fn gmp_to_native(n: &gmp::Natural) -> native::Natural {
    let mut native = native::Natural::new();
    native.assign_limbs_le(n.limbs_le().as_slice());
    native
}

pub fn native_to_gmp(n: &native::Natural) -> gmp::Natural {
    let mut gmp = gmp::Natural::new();
    gmp.assign_limbs_le(n.limbs_le().as_slice());
    gmp
}

pub fn num_to_native(n: &num::BigUint) -> native::Natural {
    native::Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_to_num(n: &native::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_to_num(n: &gmp::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rugint_to_native(n: &rugint::Integer) -> native::Natural {
    native::Natural::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_to_rugint(n: &native::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_to_rugint(n: &gmp::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}
