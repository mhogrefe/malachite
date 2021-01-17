use crate::common::{
    integer_to_bigint, integer_to_rug_integer, natural_to_biguint, natural_to_rug_integer,
};
use malachite_base_test_util::generators::common::It;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::{BigInt, BigUint};

pub fn natural_nrm(xs: It<Natural>) -> It<(BigUint, rug::Integer, Natural)> {
    Box::new(xs.map(|x| (natural_to_biguint(&x), natural_to_rug_integer(&x), x)))
}

pub fn integer_nrm(xs: It<Integer>) -> It<(BigInt, rug::Integer, Integer)> {
    Box::new(xs.map(|x| (integer_to_bigint(&x), integer_to_rug_integer(&x), x)))
}
