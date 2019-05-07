use malachite_base::comparison::Max;
use malachite_base::conversion::WrappingFrom;
use malachite_base::crement::Crementable;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{signeds_no_max, unsigneds_no_max};

fn increment_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n, out| {
        let mut n = T::from(n);
        n.increment();
        assert_eq!(n, T::from(out));
    };

    test(0, 1);
    test(1, 2);
    test(100, 101);
}

fn increment_helper_signed<T: PrimitiveSigned>() {
    let test = |n, out| {
        let mut n = T::from(n);
        n.increment();
        assert_eq!(n, T::from(out));
    };

    test(0, 1);
    test(1, 2);
    test(100, 101);
    test(-1, 0);
    test(-2, -1);
    test(-100, -99);
}

#[test]
pub fn test_increment() {
    increment_helper_unsigned::<u8>();
    increment_helper_unsigned::<u16>();
    increment_helper_unsigned::<u32>();
    increment_helper_unsigned::<u64>();
    increment_helper_signed::<i8>();
    increment_helper_signed::<i16>();
    increment_helper_signed::<i32>();
    increment_helper_signed::<i64>();
}

macro_rules! increment_fail {
    ($t:ident, $increment_fail:ident) => {
        #[test]
        #[should_panic]
        fn $increment_fail() {
            let mut n = $t::MAX;
            n.increment();
        }
    };
}

increment_fail!(u8, increment_u8_fail);
increment_fail!(u16, increment_u16_fail);
increment_fail!(u32, increment_limb_fail);
increment_fail!(u64, increment_u64_fail);
increment_fail!(i8, increment_i8_fail);
increment_fail!(i16, increment_i16_fail);
increment_fail!(i32, increment_signed_limb_fail);
increment_fail!(i64, increment_i64_fail);

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
