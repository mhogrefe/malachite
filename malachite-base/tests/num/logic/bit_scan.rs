use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
pub fn test_index_of_next_false_bit_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_false_bit(start), out);
    };
    test(0xb00000000u64, 0, Some(0));
    test(0xb00000000u64, 20, Some(20));
    test(0xb00000000u64, 31, Some(31));
    test(0xb00000000u64, 32, Some(34));
    test(0xb00000000u64, 33, Some(34));
    test(0xb00000000u64, 34, Some(34));
    test(0xb00000000u64, 35, Some(36));
    test(0xb00000000u64, 100, Some(100));

    test(0xb00000000u128, 0, Some(0));
    test(0xb00000000u128, 20, Some(20));
    test(0xb00000000u128, 31, Some(31));
    test(0xb00000000u128, 32, Some(34));
    test(0xb00000000u128, 33, Some(34));
    test(0xb00000000u128, 34, Some(34));
    test(0xb00000000u128, 35, Some(36));
    test(0xb00000000u128, 100, Some(100));
}

#[test]
pub fn test_index_of_next_true_bit_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_true_bit(start), out);
    };
    test(0xb00000000u64, 0, Some(32));
    test(0xb00000000u64, 20, Some(32));
    test(0xb00000000u64, 31, Some(32));
    test(0xb00000000u64, 32, Some(32));
    test(0xb00000000u64, 33, Some(33));
    test(0xb00000000u64, 34, Some(35));
    test(0xb00000000u64, 35, Some(35));
    test(0xb00000000u64, 36, None);
    test(0xb00000000u64, 100, None);

    test(0xb00000000u128, 0, Some(32));
    test(0xb00000000u128, 20, Some(32));
    test(0xb00000000u128, 31, Some(32));
    test(0xb00000000u128, 32, Some(32));
    test(0xb00000000u128, 33, Some(33));
    test(0xb00000000u128, 34, Some(35));
    test(0xb00000000u128, 35, Some(35));
    test(0xb00000000u128, 36, None);
    test(0xb00000000u128, 100, None);
}

#[test]
pub fn test_index_of_next_false_bit_signed() {
    fn test<T: PrimitiveSigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_false_bit(start), out);
    };
    test(-0x500000000i64, 0, Some(0));
    test(-0x500000000i64, 20, Some(20));
    test(-0x500000000i64, 31, Some(31));
    test(-0x500000000i64, 32, Some(34));
    test(-0x500000000i64, 33, Some(34));
    test(-0x500000000i64, 34, Some(34));
    test(-0x500000000i64, 35, None);
    test(-0x500000000i64, 100, None);

    test(-0x500000000i128, 0, Some(0));
    test(-0x500000000i128, 20, Some(20));
    test(-0x500000000i128, 31, Some(31));
    test(-0x500000000i128, 32, Some(34));
    test(-0x500000000i128, 33, Some(34));
    test(-0x500000000i128, 34, Some(34));
    test(-0x500000000i128, 35, None);
    test(-0x500000000i128, 100, None);
}

#[test]
pub fn test_index_of_next_true_bit_signed() {
    fn test<T: PrimitiveSigned>(x: T, start: u64, out: Option<u64>) {
        assert_eq!(x.index_of_next_true_bit(start), out);
    };
    test(-0x500000000i64, 0, Some(32));
    test(-0x500000000i64, 20, Some(32));
    test(-0x500000000i64, 31, Some(32));
    test(-0x500000000i64, 32, Some(32));
    test(-0x500000000i64, 33, Some(33));
    test(-0x500000000i64, 34, Some(35));
    test(-0x500000000i64, 35, Some(35));
    test(-0x500000000i64, 36, Some(36));
    test(-0x500000000i64, 100, Some(100));

    test(-0x500000000i128, 0, Some(32));
    test(-0x500000000i128, 20, Some(32));
    test(-0x500000000i128, 31, Some(32));
    test(-0x500000000i128, 32, Some(32));
    test(-0x500000000i128, 33, Some(33));
    test(-0x500000000i128, 34, Some(35));
    test(-0x500000000i128, 35, Some(35));
    test(-0x500000000i128, 36, Some(36));
    test(-0x500000000i128, 100, Some(100));
}
