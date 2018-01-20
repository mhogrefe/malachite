use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::conversion::from_i32::select_inputs;
use num::BigInt;
use rugint;
use std::i32;

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(i).to_string(), out);

        assert_eq!(rugint::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn from_i32_properties() {
    // from(i: i32) is valid.
    // from(i: i32) is equivalent for malachite, num, and rugint.
    // from(i: i32).to_i32() == Some(i)
    let one_i32 = |i: i32| {
        let n = Integer::from(i);
        let num_n = bigint_to_integer(&BigInt::from(i));
        let rugint_n = rugint_integer_to_integer(&rugint::Integer::from(i));
        assert!(n.is_valid());
        assert_eq!(n.to_i32(), Some(i));
        assert_eq!(n, num_n);
        assert_eq!(n, rugint_n);
    };

    for i in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_i32(i);
    }

    for i in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_i32(i);
    }
}
