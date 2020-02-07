use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::integers::_to_bits_desc_alt;
use malachite_base::num::logic::signeds::{_to_bits_asc_signed_naive, _to_bits_desc_signed_naive};
use malachite_base::num::logic::unsigneds::{
    _to_bits_asc_unsigned_naive, _to_bits_desc_unsigned_naive,
};

#[test]
pub fn test_to_bits_asc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_unsigned_naive(x), out);
    };
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[false, true]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, false, true, true, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_signed_naive(x), out);
    };
    test_signed(0i8, &[]);
    test_signed(1i16, &[true, false]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[true, true, false]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[false, true]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[true, true, false, true, true, true, true, false]);
    test_signed(
        -123i16,
        &[true, false, true, false, false, false, false, true],
    );
    test_signed(i8::MAX, &[true, true, true, true, true, true, true, false]);
    test_signed(
        i8::MIN,
        &[false, false, false, false, false, false, false, true],
    );
}

#[test]
pub fn test_to_bits_desc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_unsigned_naive(x), out);
        assert_eq!(_to_bits_desc_alt(&x), out);
    };
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[true, false]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, true, true, false, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_signed_naive(x), out);
        assert_eq!(_to_bits_desc_alt(&x), out);
    };
    test_signed(0i8, &[]);
    test_signed(1i16, &[false, true]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[false, true, true]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[true, false]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[false, true, true, true, true, false, true, true]);
    test_signed(
        -123i16,
        &[true, false, false, false, false, true, false, true],
    );
    test_signed(i8::MAX, &[false, true, true, true, true, true, true, true]);
    test_signed(
        i8::MIN,
        &[true, false, false, false, false, false, false, false],
    );
}
