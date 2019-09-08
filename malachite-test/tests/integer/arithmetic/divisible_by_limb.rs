use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
use malachite_test::common::integer_to_bigint;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{pairs_of_unsigneds, positive_unsigneds};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_positive_limb_var_1,
    pairs_of_integer_and_positive_limb_var_2, pairs_of_integer_and_unsigned,
    pairs_of_unsigned_and_integer,
};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::arithmetic::divisible_by_limb::num_divisible_by_limb;

#[test]
fn test_divisible_by_limb() {
    let test = |u, v: Limb, divisible| {
        let n = Integer::from_str(u).unwrap();
        assert_eq!(n.divisible_by(v), divisible);
        assert_eq!(n == 0 as Limb || v != 0 && n % v == 0 as Limb, divisible);

        assert_eq!(
            num_divisible_by_limb(BigInt::from_str(u).unwrap(), v),
            divisible
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(u).unwrap().is_divisible_u(v),
            divisible
        );
    };
    test("0", 0, true);
    test("1", 0, false);
    test("1000000000000", 0, false);
    test("0", 1, true);
    test("0", 123, true);
    test("1", 1, true);
    test("123", 1, true);
    test("123", 123, true);
    test("123", 456, false);
    test("456", 123, false);
    test("369", 123, true);
    test("4294967295", 1, true);
    test("4294967295", 4_294_967_295, true);
    test("1000000000000", 1, true);
    test("1000000000000", 3, false);
    test("1000000000002", 3, true);
    test("1000000000000", 123, false);
    test("1000000000000", 4_294_967_295, false);
    test("1000000000000000000000000", 1, true);
    test("1000000000000000000000000", 3, false);
    test("1000000000000000000000002", 3, true);
    test("1000000000000000000000000", 123, false);
    test("1000000000000000000000000", 4_294_967_295, false);

    test("-1", 0, false);
    test("-1000000000000", 0, false);
    test("-1", 1, true);
    test("-123", 1, true);
    test("-123", 123, true);
    test("-123", 456, false);
    test("-456", 123, false);
    test("-369", 123, true);
    test("-4294967295", 1, true);
    test("-4294967295", 4_294_967_295, true);
    test("-1000000000000", 1, true);
    test("-1000000000000", 3, false);
    test("-1000000000002", 3, true);
    test("-1000000000000", 123, false);
    test("-1000000000000", 4_294_967_295, false);
    test("-1000000000000000000000000", 1, true);
    test("-1000000000000000000000000", 3, false);
    test("-1000000000000000000000002", 3, true);
    test("-1000000000000000000000000", 123, false);
    test("-1000000000000000000000000", 4_294_967_295, false);
}

#[test]
fn test_limb_divisible_by_integer() {
    let test = |u: Limb, v, divisible| {
        let n = Integer::from_str(v).unwrap();
        assert_eq!(u.divisible_by(&n), divisible);
        assert_eq!(u == 0 || n != 0 as Limb && u % n == 0, divisible);
    };
    test(0, "0", true);
    test(1, "0", false);
    test(0, "1", true);
    test(0, "123", true);
    test(1, "1", true);
    test(123, "1", true);
    test(123, "123", true);
    test(123, "456", false);
    test(456, "123", false);
    test(369, "123", true);
    test(4_294_967_295, "1", true);
    test(4_294_967_295, "4294967295", true);
    test(0, "1000000000000", true);
    test(123, "1000000000000", false);

    test(0, "-1", true);
    test(0, "-123", true);
    test(1, "-1", true);
    test(123, "-1", true);
    test(123, "-123", true);
    test(123, "-456", false);
    test(456, "-123", false);
    test(369, "-123", true);
    test(4_294_967_295, "-1", true);
    test(4_294_967_295, "-4294967295", true);
    test(0, "-1000000000000", true);
    test(123, "-1000000000000", false);
}

fn divisible_by_limb_properties_helper(n: &Integer, u: Limb) {
    let divisible = n.divisible_by(u);
    assert_eq!(*n == 0 as Limb || u != 0 && n % u == 0 as Limb, divisible);

    //TODO assert_eq!(n.divisible_by(Integer::from(u)), remainder);

    assert_eq!(num_divisible_by_limb(integer_to_bigint(n), u), divisible);
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(integer_to_rug_integer(n).is_divisible_u(u), divisible);

    assert_eq!((-n).divisible_by(u), divisible);
}

#[test]
fn divisible_by_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            divisible_by_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_1,
        |&(ref n, u): &(Integer, Limb)| {
            assert!(n.divisible_by(u));
            assert!(*n == 0 as Limb || u != 0 && n % u == 0 as Limb);

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(num_divisible_by_limb(integer_to_bigint(n), u));
            #[cfg(feature = "32_bit_limbs")]
            assert!(integer_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_integer_and_positive_limb_var_2,
        |&(ref n, u): &(Integer, Limb)| {
            assert!(!n.divisible_by(u));
            assert!(*n != 0 as Limb && (u == 0 || n % u != 0 as Limb));

            //TODO assert!(n.divisible_by(Integer::from(u));

            assert!(!num_divisible_by_limb(integer_to_bigint(n), u));
            #[cfg(feature = "32_bit_limbs")]
            assert!(!integer_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_unsigned_and_integer,
        |&(u, ref n): &(Limb, Integer)| {
            let divisible = u.divisible_by(n);
            assert_eq!(u == 0 || *n != 0 as Limb && u % n == 0, divisible);
            assert_eq!(u.divisible_by(&-n), divisible);
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(1 as Limb));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(0 as Limb));
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert!(Integer::ZERO.divisible_by(u));
        if u > 1 {
            assert!(!Integer::ONE.divisible_by(u));
        }
        assert!(Integer::from(u).divisible_by(u));
        assert!((-Natural::from(u)).divisible_by(u));
        assert!(u.divisible_by(&Integer::from(u)));
        assert!(u.divisible_by(&-Natural::from(u)));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(divisible, Integer::from(x).divisible_by(y));
        assert_eq!(divisible, x.divisible_by(&Integer::from(y)));
    });

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert_eq!(n.divisible_by(u), Integer::from(n).divisible_by(u));
    });
}
