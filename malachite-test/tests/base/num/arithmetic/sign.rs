use std::cmp::Ordering;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_sign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&n| {
        let sign = n.sign();
        assert_ne!(sign, Ordering::Less);
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
    });
}

fn signed_sign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let sign = n.sign();
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
        if n != T::MIN {
            assert_eq!((-n).sign(), sign.reverse());
        }
    });
}

#[test]
fn sign_properties() {
    unsigned_sign_properties_helper::<u8>();
    unsigned_sign_properties_helper::<u16>();
    unsigned_sign_properties_helper::<u32>();
    unsigned_sign_properties_helper::<u64>();
    unsigned_sign_properties_helper::<usize>();
    signed_sign_properties_helper::<i8>();
    signed_sign_properties_helper::<i16>();
    signed_sign_properties_helper::<i32>();
    signed_sign_properties_helper::<i64>();
    signed_sign_properties_helper::<isize>();
}
