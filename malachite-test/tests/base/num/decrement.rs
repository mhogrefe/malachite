use common::test_properties;
use malachite_base::misc::{Min, Walkable};
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::inputs::base::{positive_unsigneds, signeds_no_min};

fn decrement_helper_unsigned<T: PrimitiveInteger>() {
    let test = |mut n: u64, out| {
        n.decrement();
        assert_eq!(T::from_u64(n), T::from_u64(out));
    };

    test(1, 0);
    test(2, 1);
    test(100, 99);
}

fn decrement_helper_signed<T: PrimitiveSigned>() {
    decrement_helper_unsigned::<T>();

    let test = |mut n: i64, out| {
        n.decrement();
        assert_eq!(T::from_i64(n), T::from_i64(out));
    };

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
    ($t: ident, $decrement_fail: ident) => {
        #[test]
        #[should_panic(expected = "Cannot decrement past the minimum value.")]
        fn $decrement_fail() {
            let mut n = $t::MIN;
            n.decrement();
        }
    }
}

decrement_fail!(u8, decrement_u8_fail);
decrement_fail!(u16, decrement_u16_fail);
decrement_fail!(u32, decrement_u32_fail);
decrement_fail!(u64, decrement_u64_fail);
decrement_fail!(i8, decrement_i8_fail);
decrement_fail!(i16, decrement_i16_fail);
decrement_fail!(i32, decrement_i32_fail);
decrement_fail!(i64, decrement_i64_fail);

fn decrement_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let mut n_mut = n;
        n_mut.decrement();
        assert_ne!(n_mut, n);
        n_mut.increment();
        assert_eq!(n_mut, n);
    });
}

fn decrement_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    test_properties(signeds_no_min, |&n: &T| {
        let mut n_mut = n;
        n_mut.decrement();
        assert_ne!(n_mut, n);
        n_mut.increment();
        assert_eq!(n_mut, n);
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
