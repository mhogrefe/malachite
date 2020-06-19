use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_unsigned, pairs_of_small_unsigneds,
};

fn unsigned_wrapping_pow_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties_no_special(pairs_of_small_unsigneds::<T, u64>, |&(x, y)| {
        let mut power = x;
        power.wrapping_pow_assign(y);
        assert_eq!(power, x.wrapping_pow(y));
    });
}

fn signed_wrapping_pow_assign_properties_helper<T: PrimitiveSigned + Rand>() {
    test_properties_no_special(
        pairs_of_small_signed_and_small_unsigned::<T, u64>,
        |&(x, y)| {
            let mut power = x;
            power.wrapping_pow_assign(y);
            assert_eq!(power, x.wrapping_pow(y));
            if x != T::MIN {
                let neg_pow = (-x).wrapping_pow(y);
                if y.even() {
                    assert_eq!(neg_pow, power);
                } else {
                    assert_eq!(neg_pow, power.wrapping_neg());
                }
            }
        },
    );
}

#[test]
fn wrapping_pow_assign_properties() {
    unsigned_wrapping_pow_assign_properties_helper::<u8>();
    unsigned_wrapping_pow_assign_properties_helper::<u16>();
    unsigned_wrapping_pow_assign_properties_helper::<u32>();
    unsigned_wrapping_pow_assign_properties_helper::<u64>();
    unsigned_wrapping_pow_assign_properties_helper::<usize>();

    signed_wrapping_pow_assign_properties_helper::<i8>();
    signed_wrapping_pow_assign_properties_helper::<i16>();
    signed_wrapping_pow_assign_properties_helper::<i32>();
    signed_wrapping_pow_assign_properties_helper::<i64>();
    signed_wrapping_pow_assign_properties_helper::<isize>();
}
