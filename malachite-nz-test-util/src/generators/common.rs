use crate::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_base_test_util::generators::common::It;
use malachite_nz::natural::Natural;
use num::BigUint;

pub fn natural_nrm(xs: It<Natural>) -> It<(BigUint, rug::Integer, Natural)> {
    Box::new(xs.map(|x| (natural_to_biguint(&x), natural_to_rug_integer(&x), x)))
}
