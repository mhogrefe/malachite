use malachite_base::num::basic::traits::Iverson;

macro_rules! test_iverson {
    ($t: ident) => {
        assert_eq!($t::iverson(false), 0);
        assert_eq!($t::iverson(true), 1);
    };
}

#[test]
fn test_iverson() {
    test_iverson!(u8);
    test_iverson!(u16);
    test_iverson!(u32);
    test_iverson!(u64);
    test_iverson!(u128);
    test_iverson!(usize);
    test_iverson!(i8);
    test_iverson!(i16);
    test_iverson!(i32);
    test_iverson!(i64);
    test_iverson!(i128);
    test_iverson!(isize);
}
