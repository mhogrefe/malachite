use std::fmt::Debug;

use malachite_base_test_util::generators::{signed_gen, unsigned_gen};

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom};

#[test]
pub fn test_convertible_from() {
    fn test_single<T: ConvertibleFrom<T> + Copy + Debug>(n: T) {
        assert!(T::convertible_from(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: ConvertibleFrom<T>>(n_in: T, convertible: bool) {
        assert_eq!(U::convertible_from(n_in), convertible);
    };
    test_double::<_, u16>(0u8, true);
    test_double::<_, i32>(1000u16, true);
    test_double::<_, i8>(-5i16, true);
    test_double::<_, u64>(255u8, true);

    test_double::<_, u32>(-1i8, false);
    test_double::<_, u16>(u32::MAX, false);
    test_double::<_, u32>(i32::MIN, false);
    test_double::<_, u16>(i32::MIN, false);
    test_double::<_, i16>(i32::MIN, false);
    test_double::<_, u32>(-5i32, false);
    test_double::<_, i32>(3000000000u32, false);
    test_double::<_, i8>(-1000i16, false);
}

fn convertible_from_helper_primitive_int_unsigned<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveUnsigned,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let convertible = T::convertible_from(u);
        assert_eq!(convertible, T::checked_from(u).is_some())
    });
}

fn convertible_from_helper_primitive_int_signed<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveSigned,
>() {
    signed_gen::<U>().test_properties(|i| {
        let convertible = T::convertible_from(i);
        assert_eq!(convertible, T::checked_from(i).is_some())
    });
}

#[test]
fn convertible_from_properties() {
    apply_fn_to_primitive_ints_and_unsigneds!(convertible_from_helper_primitive_int_unsigned);
    apply_fn_to_primitive_ints_and_signeds!(convertible_from_helper_primitive_int_signed);
}
