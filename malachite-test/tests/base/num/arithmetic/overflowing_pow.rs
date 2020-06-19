use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_unsigned, pairs_of_small_unsigneds,
};

fn unsigned_overflowing_pow_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties_no_special(pairs_of_small_unsigneds::<T, u64>, |&(x, y)| {
        let mut power = x;
        let overflow = power.overflowing_pow_assign(y);
        assert_eq!((power, overflow), x.overflowing_pow(y));
        assert_eq!(x.wrapping_pow(y), power);
        if !overflow {
            assert_eq!(power, x.pow(y));
        }
    });
}

fn signed_overflowing_pow_assign_properties_helper<T: PrimitiveSigned + Rand>() {
    test_properties_no_special(
        pairs_of_small_signed_and_small_unsigned::<T, u64>,
        |&(x, y)| {
            let mut power = x;
            let overflow = power.overflowing_pow_assign(y);
            assert_eq!((power, overflow), x.overflowing_pow(y));
            assert_eq!(x.wrapping_pow(y), power);
            if !overflow {
                assert_eq!(power, x.pow(y));
            }
        },
    );
}

#[test]
fn overflowing_pow_assign_properties() {
    unsigned_overflowing_pow_assign_properties_helper::<u8>();
    unsigned_overflowing_pow_assign_properties_helper::<u16>();
    unsigned_overflowing_pow_assign_properties_helper::<u32>();
    unsigned_overflowing_pow_assign_properties_helper::<u64>();
    unsigned_overflowing_pow_assign_properties_helper::<usize>();

    signed_overflowing_pow_assign_properties_helper::<i8>();
    signed_overflowing_pow_assign_properties_helper::<i16>();
    signed_overflowing_pow_assign_properties_helper::<i32>();
    signed_overflowing_pow_assign_properties_helper::<i64>();
    signed_overflowing_pow_assign_properties_helper::<isize>();
}
