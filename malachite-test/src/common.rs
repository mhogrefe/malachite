use malachite_native as native;
use malachite_gmp as gmp;
use num;
use rugint;
use std::str::FromStr;

pub fn gmp_natural_to_native(n: &gmp::natural::Natural) -> native::natural::Natural {
    let mut native = native::natural::Natural::new();
    native.assign_limbs_le(n.limbs_le().as_slice());
    native
}

pub fn native_natural_to_gmp(n: &native::natural::Natural) -> gmp::natural::Natural {
    let mut gmp = gmp::natural::Natural::new();
    gmp.assign_limbs_le(n.limbs_le().as_slice());
    gmp
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
    let mut native = native::natural::Natural::new();
    //TODO use better unsigned_abs
    native.assign_limbs_le(n.clone()
                               .unsigned_abs()
                               .limbs_le()
                               .as_slice());
    if n >= &0 {
        native.into_integer()
    } else {
        -native.into_integer()
    }
}

pub fn native_integer_to_gmp(n: &gmp::integer::Integer) -> gmp::integer::Integer {
    let mut native = gmp::natural::Natural::new();
    //TODO use better unsigned_abs
    native.assign_limbs_le(n.clone()
                               .unsigned_abs()
                               .limbs_le()
                               .as_slice());
    if n >= &0 {
        native.into_integer()
    } else {
        -native.into_integer()
    }
}

pub fn num_bigint_to_native_integer(n: &num::BigInt) -> native::integer::Integer {
    native::integer::Integer::from_str(n.to_string().as_ref()).unwrap()
}

pub fn native_integer_to_num_bigint(n: &native::natural::Natural) -> num::BigInt {
    num::BigInt::from_str(n.to_string().as_ref()).unwrap()
}

pub fn gmp_integer_to_num_bigint(n: &gmp::natural::Natural) -> num::BigInt {
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
