use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::bool_gen;
use std::fmt::Debug;
use std::ops::Sub;

macro_rules! test_iverson_primitive_int {
    ($t: ident) => {
        assert_eq!($t::iverson(false), 0);
        assert_eq!($t::iverson(true), 1);
    };
}

macro_rules! test_iverson_primitive_float {
    ($t: ident) => {
        assert_eq!($t::iverson(false), 0.0);
        assert_eq!($t::iverson(true), 1.0);
    };
}

#[test]
fn test_iverson() {
    apply_to_primitive_ints!(test_iverson_primitive_int);
    apply_to_primitive_floats!(test_iverson_primitive_float);
}

fn iverson_properties_helper<
    T: Copy + Debug + PartialEq + Iverson + One + Sub<Output = T> + Zero,
>() {
    bool_gen().test_properties(|b| {
        let n = T::iverson(b);
        assert!(n == T::ZERO || n == T::ONE);
        assert_eq!(T::iverson(!b), T::ONE - n);
    });
}

#[test]
fn iverson_properties() {
    apply_fn_to_primitive_ints!(iverson_properties_helper);
    apply_fn_to_primitive_floats!(iverson_properties_helper);
}
