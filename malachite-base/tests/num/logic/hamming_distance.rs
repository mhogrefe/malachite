use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
pub fn test_hamming_distance() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: u64) {
        assert_eq!(x.hamming_distance(y), out);
    };

    test(123u16, 456u16, 6);
    test(0xffffu32, 0xffff_0000u32, 32);
    test(0xffffu32, u32::MAX, 16);
    test(0xffff_0000u32, u32::MAX, 16);
}

#[test]
pub fn test_checked_hamming_distance() {
    fn test<T: PrimitiveSigned>(x: T, y: T, out: Option<u64>) {
        assert_eq!(x.checked_hamming_distance(y), out);
    };

    test(123i32, 456i32, Some(6));
    test(-123i32, -456i32, Some(7));
    test(0i8, 127i8, Some(7));
    test(0i8, -1i8, None);
    test(-1i8, -128i8, Some(7));
    test(0i128, i128::MAX, Some(127));
    test(0i128, -1i128, None);
    test(-1i128, i128::MIN, Some(127));
    test(0xffffi32, 0x7fff_0000i32, Some(31));
    test(0xffffi32, i32::MAX, Some(15));
    test(0x7fff_0000i32, i32::MAX, Some(16));
}
