use common::LARGE_LIMIT;
use malachite_base::traits::NegAssign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::arithmetic::neg::select_inputs;
use num::BigInt;
use rugint;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&Integer::from_str(s).unwrap());
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rugint::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
        x.neg_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    // -x is equivalent for malachite, num, and rugint.
    // -x is valid.
    //
    // -&x is equivalent for malachite, num, and rugint.
    // -&x is valid.
    // -x and -&x are equivalent.
    //
    // (-x == x) == (x == 0)
    // --x == x
    let one_integer = |x: Integer| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        let num_neg = -integer_to_bigint(&x);
        assert_eq!(bigint_to_integer(&num_neg), neg);

        let rugint_neg = -integer_to_rugint_integer(&x);
        assert_eq!(rugint_integer_to_integer(&rugint_neg), neg);

        let neg_2 = -&x;
        assert!(neg_2.is_valid());

        assert_eq!(neg_2, neg);

        assert_eq!(neg == x, x == 0);
        assert_eq!(-&neg, x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
