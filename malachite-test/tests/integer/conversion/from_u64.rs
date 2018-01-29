use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, GenerationMode};
use malachite_test::inputs::base::unsigneds;
use num::BigInt;
use std::u64;

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(u).to_string(), out);
    };
    test(0u64, "0");
    test(123u64, "123");
    test(1_000_000_000_000u64, "1000000000000");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn from_u64_properties() {
    // from(u: u64) is valid.
    // from(u: u64) is equivalent for malachite and num.
    // from(u: u64).to_u64() == Some(u)
    let one_u64 = |u: u64| {
        let n = Integer::from(u);
        let num_n = bigint_to_integer(&BigInt::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u64(), Some(u));
        assert_eq!(n, num_n);
    };

    for u in unsigneds::<u64>(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u64(u);
    }

    for u in unsigneds::<u64>(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u64(u);
    }
}
