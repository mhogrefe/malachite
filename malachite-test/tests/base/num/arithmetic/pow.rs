use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_u64_var_2, pairs_of_small_unsigneds_var_2,
};

fn unsigned_pow_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties_no_special(pairs_of_small_unsigneds_var_2::<T>, |&(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
    });
}

fn signed_pow_assign_properties_helper<T: PrimitiveSigned + Rand>() {
    test_properties_no_special(pairs_of_small_signed_and_small_u64_var_2::<T>, |&(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
    });
}

#[test]
fn pow_assign_properties() {
    unsigned_pow_assign_properties_helper::<u8>();
    unsigned_pow_assign_properties_helper::<u16>();
    unsigned_pow_assign_properties_helper::<u32>();
    unsigned_pow_assign_properties_helper::<u64>();
    unsigned_pow_assign_properties_helper::<usize>();

    signed_pow_assign_properties_helper::<i8>();
    signed_pow_assign_properties_helper::<i16>();
    signed_pow_assign_properties_helper::<i32>();
    signed_pow_assign_properties_helper::<i64>();
    signed_pow_assign_properties_helper::<isize>();
}
