use std::cmp::min;

use malachite_base::num::arithmetic::traits::{
    Abs, CeilingModPowerOf2, CeilingModPowerOf2Assign, DivisibleByPowerOf2, ModPowerOf2,
    ModPowerOf2Assign, NegModPowerOf2, PowerOf2, RemPowerOf2, RemPowerOf2Assign, ShrRound, Sign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_u64_var_2, pairs_of_signed_and_small_u64_var_4,
    pairs_of_signed_and_small_unsigned, unsigneds,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, pairs_of_integer_and_small_unsigned_var_1,
    pairs_of_integer_and_small_unsigned_var_2, triples_of_integer_integer_and_small_unsigned,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn mod_power_of_2_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).mod_power_of_2(u), result);
        assert_eq!(n & Integer::low_mask(u), result);
    });

    test_properties(
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, u)| {
            let xm = Integer::from(x.mod_power_of_2(u));
            let ym = Integer::from(y.mod_power_of_2(u));
            assert_eq!((x + y).mod_power_of_2(u), (&xm + &ym).mod_power_of_2(u));
            assert_eq!((x - y).mod_power_of_2(u), (&xm - &ym).mod_power_of_2(u));
            assert_eq!((x * y).mod_power_of_2(u), (xm * ym).mod_power_of_2(u));
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_2(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_2(u), 0);
        assert_eq!(
            Integer::from(n.mod_power_of_2(u)) - n.ceiling_mod_power_of_2(u),
            Natural::power_of_2(u)
        );
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_2(u).mod_power_of_2(v),
                n.mod_power_of_2(min(u, v))
            );
        },
    );

    test_properties(
        pairs_of_signed_and_small_u64_var_2::<SignedLimb>,
        |&(i, pow)| {
            assert_eq!(i.mod_power_of_2(pow), Integer::from(i).mod_power_of_2(pow));
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(n.mod_power_of_2(pow), Integer::from(n).mod_power_of_2(pow));
    });

    test_properties(integers, |n| {
        assert_eq!(n.mod_power_of_2(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.mod_power_of_2(u), 0);
    });
}

#[test]
fn rem_power_of_2_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.rem_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert!(result.le_abs(n));
        assert_eq!(((n.shr_round(u, RoundingMode::Down) << u) + &result), *n);
        assert!(result.lt_abs(&Natural::power_of_2(u)));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).rem_power_of_2(u), result);
        assert_eq!(n.abs().mod_power_of_2(u), result.abs());
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.rem_power_of_2(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.rem_power_of_2(u), 0);
        assert_eq!(n.rem_power_of_2(u).sign(), n.sign());
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.rem_power_of_2(u).rem_power_of_2(v),
                n.rem_power_of_2(min(u, v))
            );
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, pow)| {
            assert_eq!(i.rem_power_of_2(pow), Integer::from(i).rem_power_of_2(pow));
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(n.rem_power_of_2(pow), Integer::from(n).rem_power_of_2(pow));
    });

    test_properties(integers, |n| {
        assert_eq!(n.rem_power_of_2(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.rem_power_of_2(u), 0);
    });
}

#[test]
fn ceiling_mod_power_of_2_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.ceiling_mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).ceiling_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().ceiling_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((n.shr_round(u, RoundingMode::Ceiling) << u) + &result), *n);
        assert!(result <= 0);
        assert!(-&result <= Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((-n).mod_power_of_2(u), -result);
    });

    test_properties(
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, u)| {
            let xm = Integer::from(x.mod_power_of_2(u));
            let ym = Integer::from(y.mod_power_of_2(u));
            assert_eq!(
                (x + y).ceiling_mod_power_of_2(u),
                Integer::from(&xm + &ym).ceiling_mod_power_of_2(u)
            );
            assert_eq!(
                (x - y).ceiling_mod_power_of_2(u),
                Integer::from(&xm - &ym).ceiling_mod_power_of_2(u)
            );
            assert_eq!(
                (x * y).ceiling_mod_power_of_2(u),
                Integer::from(xm * ym).ceiling_mod_power_of_2(u)
            );
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.ceiling_mod_power_of_2(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.ceiling_mod_power_of_2(u), 0);
    });

    test_properties(
        pairs_of_signed_and_small_u64_var_4::<SignedLimb>,
        |&(i, pow)| {
            assert_eq!(
                i.ceiling_mod_power_of_2(pow),
                Integer::from(i).ceiling_mod_power_of_2(pow)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            -n.neg_mod_power_of_2(pow),
            Integer::from(n).ceiling_mod_power_of_2(pow)
        );
    });

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod_power_of_2(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.ceiling_mod_power_of_2(u), 0);
    });
}
