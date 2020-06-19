use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_unsigned, pairs_of_small_unsigneds,
};

fn unsigned_saturating_pow_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties_no_special(pairs_of_small_unsigneds::<T, u64>, |&(x, y)| {
        let mut power = x;
        power.saturating_pow_assign(y);
        assert_eq!(power, x.saturating_pow(y));
        if y != 0 {
            assert!(power >= x);
        }
        if power < T::MAX {
            assert_eq!(power, x.pow(y));
        }
    });
}

fn signed_saturating_pow_assign_properties_helper<T: PrimitiveSigned + Rand>() {
    test_properties_no_special(
        pairs_of_small_signed_and_small_unsigned::<T, u64>,
        |&(x, y)| {
            let mut power = x;
            power.saturating_pow_assign(y);
            assert_eq!(power, x.saturating_pow(y));
            if power > T::MIN && power < T::MAX {
                assert_eq!(power, x.pow(y));
            }
        },
    );
}

#[test]
fn saturating_pow_assign_properties() {
    unsigned_saturating_pow_assign_properties_helper::<u8>();
    unsigned_saturating_pow_assign_properties_helper::<u16>();
    unsigned_saturating_pow_assign_properties_helper::<u32>();
    unsigned_saturating_pow_assign_properties_helper::<u64>();
    unsigned_saturating_pow_assign_properties_helper::<usize>();

    signed_saturating_pow_assign_properties_helper::<i8>();
    signed_saturating_pow_assign_properties_helper::<i16>();
    signed_saturating_pow_assign_properties_helper::<i32>();
    signed_saturating_pow_assign_properties_helper::<i64>();
    signed_saturating_pow_assign_properties_helper::<isize>();
}
