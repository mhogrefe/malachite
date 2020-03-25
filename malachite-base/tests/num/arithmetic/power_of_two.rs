use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn power_of_two_primitive_helper<T: PrimitiveInteger>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_two(pow), out);
    };
    test(0, T::ONE);
    test(1, T::TWO);
    test(2, T::exact_from(4));
    test(3, T::exact_from(8));
}

fn power_of_two_unsigned_helper<T: PrimitiveUnsigned>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_two(pow), out);
    };
    test(T::WIDTH - 1, T::ONE << (T::WIDTH - 1));
}

#[test]
fn test_power_of_two() {
    power_of_two_primitive_helper::<u8>();
    power_of_two_primitive_helper::<u16>();
    power_of_two_primitive_helper::<u32>();
    power_of_two_primitive_helper::<u64>();
    power_of_two_primitive_helper::<u128>();
    power_of_two_primitive_helper::<usize>();
    power_of_two_primitive_helper::<i8>();
    power_of_two_primitive_helper::<i16>();
    power_of_two_primitive_helper::<i32>();
    power_of_two_primitive_helper::<i64>();
    power_of_two_primitive_helper::<i128>();
    power_of_two_primitive_helper::<isize>();

    power_of_two_unsigned_helper::<u8>();
    power_of_two_unsigned_helper::<u16>();
    power_of_two_unsigned_helper::<u32>();
    power_of_two_unsigned_helper::<u64>();
    power_of_two_unsigned_helper::<u128>();
    power_of_two_unsigned_helper::<usize>();
}

macro_rules! power_of_two_unsigned_fail {
    ($t:ident, $power_of_two_fail:ident) => {
        #[test]
        #[should_panic]
        fn $power_of_two_fail() {
            $t::power_of_two($t::WIDTH);
        }
    };
}

power_of_two_unsigned_fail!(u8, power_of_two_u8_fail);
power_of_two_unsigned_fail!(u16, power_of_two_u16_fail);
power_of_two_unsigned_fail!(u32, power_of_two_u32_fail);
power_of_two_unsigned_fail!(u64, power_of_two_u64_fail);
power_of_two_unsigned_fail!(u128, power_of_two_u128_fail);
power_of_two_unsigned_fail!(usize, power_of_two_usize_fail);

macro_rules! floor_log_two_signed_fail {
    ($t:ident, $floor_log_two_fail:ident) => {
        #[test]
        #[should_panic]
        fn $floor_log_two_fail() {
            $t::power_of_two($t::WIDTH - 1);
        }
    };
}

floor_log_two_signed_fail!(i8, floor_log_two_u8_fail);
floor_log_two_signed_fail!(i16, floor_log_two_u16_fail);
floor_log_two_signed_fail!(i32, floor_log_two_u32_fail);
floor_log_two_signed_fail!(i64, floor_log_two_u64_fail);
floor_log_two_signed_fail!(i128, floor_log_two_u128_fail);
floor_log_two_signed_fail!(isize, floor_log_two_usize_fail);
