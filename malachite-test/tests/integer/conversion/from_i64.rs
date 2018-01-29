use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, GenerationMode};
use malachite_test::inputs::base::signeds;
use num::BigInt;
use std::i64;

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(i).to_string(), out);
    };
    test(0i64, "0");
    test(123i64, "123");
    test(-123i64, "-123");
    test(1_000_000_000_000i64, "1000000000000");
    test(-1_000_000_000_000i64, "-1000000000000");
    test(i64::MAX, "9223372036854775807");
    test(i64::MIN, "-9223372036854775808");
}

#[test]
fn from_i64_properties() {
    // from(i: i64) is valid.
    // from(i: i64) is equivalent for malachite and num.
    // from(i: i64).to_u64() == Some(i)
    let one_i64 = |i: i64| {
        let n = Integer::from(i);
        let num_n = bigint_to_integer(&BigInt::from(i));
        assert!(n.is_valid());
        assert_eq!(n.to_i64(), Some(i));
        assert_eq!(n, num_n);
    };

    for i in signeds::<i64>(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_i64(i);
    }

    for i in signeds::<i64>(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_i64(i);
    }
}
