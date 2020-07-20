use malachite_base::num::basic::traits::Iverson;

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
