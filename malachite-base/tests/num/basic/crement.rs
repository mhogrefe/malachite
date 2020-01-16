use malachite_base::comparison::{Max, Min};
use malachite_base::crement::Crementable;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

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
fn test_increment() {
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
fn test_decrement() {
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
