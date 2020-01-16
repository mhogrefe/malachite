use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    positive_unsigneds, signeds_no_max, signeds_no_min, unsigneds_no_max,
};

fn increment_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds_no_max, |&n: &T| {
        let mut mut_n = n;
        mut_n.increment();
        assert_ne!(mut_n, n);
        mut_n.decrement();
        assert_eq!(mut_n, n);
    });
}

fn increment_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds_no_max, |&n: &T| {
        let mut mut_n = n;
        mut_n.increment();
        assert_ne!(mut_n, n);
        mut_n.decrement();
        assert_eq!(mut_n, n);
    });
}

#[test]
fn increment_properties() {
    increment_properties_helper_unsigned::<u8>();
    increment_properties_helper_unsigned::<u16>();
    increment_properties_helper_unsigned::<u32>();
    increment_properties_helper_unsigned::<u64>();
    increment_properties_helper_signed::<i8>();
    increment_properties_helper_signed::<i16>();
    increment_properties_helper_signed::<i32>();
    increment_properties_helper_signed::<i64>();
}

fn decrement_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let mut mut_n = n;
        mut_n.decrement();
        assert_ne!(mut_n, n);
        mut_n.increment();
        assert_eq!(mut_n, n);
    });
}

fn decrement_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds_no_min, |&n: &T| {
        let mut mut_n = n;
        mut_n.decrement();
        assert_ne!(mut_n, n);
        mut_n.increment();
        assert_eq!(mut_n, n);
    });
}

#[test]
fn decrement_properties() {
    decrement_properties_helper_unsigned::<u8>();
    decrement_properties_helper_unsigned::<u16>();
    decrement_properties_helper_unsigned::<u32>();
    decrement_properties_helper_unsigned::<u64>();
    decrement_properties_helper_signed::<i8>();
    decrement_properties_helper_signed::<i16>();
    decrement_properties_helper_signed::<i32>();
    decrement_properties_helper_signed::<i64>();
}
