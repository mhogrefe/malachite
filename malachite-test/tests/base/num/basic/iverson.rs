use malachite_base::num::basic::integers::PrimitiveInteger;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::bools;

fn iverson_properties_helper<T: PrimitiveInteger>() {
    test_properties_no_special(bools, |&b| {
        let n = T::iverson(b);
        assert!(n == T::ZERO || n == T::ONE);
        assert_eq!(T::iverson(!b), T::ONE - n);
    });
}

#[test]
fn iverson_properties() {
    iverson_properties_helper::<u8>();
    iverson_properties_helper::<u16>();
    iverson_properties_helper::<u32>();
    iverson_properties_helper::<u64>();
    iverson_properties_helper::<u128>();
    iverson_properties_helper::<usize>();
    iverson_properties_helper::<i8>();
    iverson_properties_helper::<i16>();
    iverson_properties_helper::<i32>();
    iverson_properties_helper::<i64>();
    iverson_properties_helper::<i128>();
    iverson_properties_helper::<isize>();
}
