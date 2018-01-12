use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::conversion::from_u32::select_inputs;
use num::BigInt;
use rugint;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(u).to_string(), out);

        assert_eq!(rugint::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::max_value(), "4294967295");
}

#[test]
fn from_u32_properties() {
    // from(u: u32) is valid.
    // from(u: u32) is equivalent for malachite, num, and rugint.
    // from(u: u32).to_u32() == Some(u)
    let one_u32 = |u: u32| {
        let n = Integer::from(u);
        let num_n = bigint_to_integer(&BigInt::from(u));
        let rugint_n = rugint_integer_to_integer(&rugint::Integer::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(n, num_n);
        assert_eq!(n, rugint_n);
    };

    for u in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
