use common::test_properties;
use malachite_base::misc::{Min, Walkable};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::inputs::base::{positive_unsigneds, signeds_no_min};
use rand::Rand;

fn decrement_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n, out| {
        let mut n = T::from(n);
        n.decrement();
        assert_eq!(n, T::from(out));
    };

    test(1, 0);
    test(2, 1);
    test(100, 99);
}

fn decrement_helper_signed<T: PrimitiveSigned>() {
    let test = |n, out| {
        let mut n = T::from(n);
        n.decrement();
        assert_eq!(n, T::from(out));
    };

    test(1, 0);
    test(2, 1);
    test(100, 99);
    test(0, -1);
    test(-1, -2);
    test(-100, -101);
}

#[test]
pub fn test_decrement() {
    decrement_helper_unsigned::<u8>();
    decrement_helper_unsigned::<u16>();
    decrement_helper_unsigned::<u32>();
    decrement_helper_unsigned::<u64>();
    decrement_helper_signed::<i8>();
    decrement_helper_signed::<i16>();
    decrement_helper_signed::<i32>();
    decrement_helper_signed::<i64>();
}

macro_rules! decrement_fail {
    ($t:ident, $decrement_fail:ident) => {
        #[test]
        #[should_panic]
        fn $decrement_fail() {
            let mut n = $t::MIN;
            n.decrement();
        }
    };
}

decrement_fail!(u8, decrement_u8_fail);
decrement_fail!(u16, decrement_u16_fail);
decrement_fail!(u32, decrement_limb_fail);
decrement_fail!(u64, decrement_u64_fail);
decrement_fail!(i8, decrement_i8_fail);
decrement_fail!(i16, decrement_i16_fail);
decrement_fail!(i32, decrement_signed_limb_fail);
decrement_fail!(i64, decrement_i64_fail);

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
