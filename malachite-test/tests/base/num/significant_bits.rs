use common::LARGE_LIMIT;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{signeds, unsigneds};

fn significant_bits_helper_common<T: PrimitiveInteger>() {
    let test = |n, out| {
        assert_eq!(T::from_u64(n).significant_bits(), out);
    };

    test(0, 0);
    test(1, 1);
    test(2, 2);
    test(3, 2);
    test(4, 3);
    test(5, 3);
    test(100, 7);
}

fn significant_bits_helper_unsigned<T: PrimitiveUnsigned>(max: u64) {
    significant_bits_helper_common::<T>();

    let test = |n, out| {
        assert_eq!(T::from_u64(n).significant_bits(), out);
    };

    test(max, T::WIDTH.into());
}

fn significant_bits_helper_signed<T: PrimitiveSigned>(max: i64, min: i64) {
    significant_bits_helper_common::<T>();

    let test = |n, out| {
        assert_eq!(T::from_i64(n).significant_bits(), out);
    };

    let width = T::WIDTH.into();
    test(max, width - 1);
    test(min, width);
}

#[test]
pub fn test_significant_bits() {
    significant_bits_helper_unsigned::<u8>(u8::MAX.into());
    significant_bits_helper_unsigned::<u16>(u16::MAX.into());
    significant_bits_helper_unsigned::<u32>(u32::MAX.into());
    significant_bits_helper_unsigned::<u64>(u64::MAX);
    significant_bits_helper_signed::<i8>(i8::MAX.into(), i8::MIN.into());
    significant_bits_helper_signed::<i16>(i16::MAX.into(), i16::MIN.into());
    significant_bits_helper_signed::<i32>(i32::MAX.into(), i32::MIN.into());
    significant_bits_helper_signed::<i64>(i64::MAX, i64::MIN);
}

fn significant_bits_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    // n.significant_bits() <= T::WIDTH
    // n.significant_bits() == 0 iff n == 0
    let unsigned = |n: T| {
        let significant_bits = n.significant_bits();
        assert!(significant_bits <= T::WIDTH.into());
        assert_eq!(significant_bits == 0, n == T::ZERO);
    };

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        unsigned(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        unsigned(n);
    }
}

fn significant_bits_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    // n.significant_bits() <= T::WIDTH
    // n.significant_bits() == 0 iff n == 0
    // n.significant_bits() == T::WIDTH iff n == T::MIN
    // n.significant_bits() == n.wrapping_neg().significant_bits()
    let signed = |n: T| {
        let significant_bits = n.significant_bits();
        assert!(significant_bits <= T::WIDTH.into());
        assert_eq!(significant_bits == 0, n == T::ZERO);
        assert_eq!(significant_bits == T::WIDTH.into(), n == T::MIN);
        assert_eq!(n.wrapping_neg().significant_bits(), significant_bits);
    };

    for n in signeds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        signed(n);
    }

    for n in signeds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        signed(n);
    }
}

#[test]
fn significant_bits_properties() {
    significant_bits_properties_helper_unsigned::<u8>();
    significant_bits_properties_helper_unsigned::<u16>();
    significant_bits_properties_helper_unsigned::<u32>();
    significant_bits_properties_helper_unsigned::<u64>();
    significant_bits_properties_helper_signed::<i8>();
    significant_bits_properties_helper_signed::<i16>();
    significant_bits_properties_helper_signed::<i32>();
    significant_bits_properties_helper_signed::<i64>();
}
