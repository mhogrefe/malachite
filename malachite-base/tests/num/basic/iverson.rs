use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Iverson;
use malachite_base_test_util::generators::bool_gen;

macro_rules! test_iverson {
    ($t: ident) => {
        assert_eq!($t::iverson(false), 0);
        assert_eq!($t::iverson(true), 1);
    };
}

#[test]
fn test_iverson() {
    apply_to_primitive_ints!(test_iverson);
}

fn iverson_properties_helper<T: PrimitiveInt>() {
    bool_gen().test_properties(|b| {
        let n = T::iverson(b);
        assert!(n == T::ZERO || n == T::ONE);
        assert_eq!(T::iverson(!b), T::ONE - n);
    });
}

#[test]
fn iverson_properties() {
    apply_fn_to_primitive_ints!(iverson_properties_helper);
}
