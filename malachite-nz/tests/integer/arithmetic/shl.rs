use malachite_base::num::arithmetic::traits::{Abs, IsPowerOf2, PowerOf2, ShlRound, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{signed_gen, unsigned_gen_var_5};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{
    integer_gen, integer_signed_pair_gen_var_1, integer_unsigned_pair_gen_var_2,
    natural_signed_pair_gen_var_2, natural_unsigned_pair_gen_var_4,
};
use num::BigInt;
use rug;
use std::ops::{Shl, ShlAssign, Shr};
use std::str::FromStr;

macro_rules! tests_unsigned {
    (
        $t:ident,
        $test_shl_u:ident,
        $u:ident,
        $v:ident,
        $out:ident,
        $library_comparison_tests:expr
    ) => {
        #[test]
        fn $test_shl_u() {
            let test = |$u, $v: $t, $out| {
                let u = Integer::from_str($u).unwrap();

                let mut n = u.clone();
                n <<= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = u.clone() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &u << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");

            test("123", 0, "123");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );

            test("-123", 0, "-123");
            test("-123", 1, "-246");
            test("-123", 2, "-492");
            test("-123", 25, "-4127195136");
            test("-123", 26, "-8254390272");
            test("-123", 100, "-155921023828072216384094494261248");
            test("-2147483648", 1, "-4294967296");
            test("-1000000000000", 0, "-1000000000000");
            test("-1000000000000", 3, "-8000000000000");
            test("-1000000000000", 24, "-16777216000000000000");
            test("-1000000000000", 25, "-33554432000000000000");
            test("-1000000000000", 31, "-2147483648000000000000");
            test("-1000000000000", 32, "-4294967296000000000000");
            test("-1000000000000", 33, "-8589934592000000000000");
            test(
                "-1000000000000",
                100,
                "-1267650600228229401496703205376000000000000",
            );
        }
    };
}
tests_unsigned!(u8, test_shl_u8, u, v, out, {});
tests_unsigned!(u16, test_shl_u16, u, v, out, {});
tests_unsigned!(u32, test_shl_limb, u, v, out, {
    let mut n = rug::Integer::from_str(u).unwrap();
    n <<= v;
    assert_eq!(n.to_string(), out);

    let n = rug::Integer::from_str(u).unwrap() << v;
    assert_eq!(n.to_string(), out);

    let n = BigInt::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);

    let n = &BigInt::from_str(u).unwrap() << usize::exact_from(v);
    assert_eq!(n.to_string(), out);
});
tests_unsigned!(u64, test_shl_u64, u, v, out, {});
tests_unsigned!(u128, test_shl_u128, u, v, out, {});
tests_unsigned!(usize, test_shl_usize, u, v, out, {});

macro_rules! tests_signed {
    (
        $t:ident,
        $test_shl_i:ident,
        $i:ident,
        $j:ident,
        $out:ident,
        $shl_library_comparison_tests:expr,
        $n:ident
    ) => {
        #[test]
        fn $test_shl_i() {
            let test = |$i, $j: $t, $out| {
                let u = Integer::from_str($i).unwrap();

                let mut n = u.clone();
                n <<= $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = u.clone() << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &u << $j;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $shl_library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("0", 10, "0");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );

            test("-123", 1, "-246");
            test("-123", 2, "-492");
            test("-123", 25, "-4127195136");
            test("-123", 26, "-8254390272");
            test("-123", 100, "-155921023828072216384094494261248");
            test("-2147483648", 1, "-4294967296");
            test("-1000000000000", 3, "-8000000000000");
            test("-1000000000000", 24, "-16777216000000000000");
            test("-1000000000000", 25, "-33554432000000000000");
            test("-1000000000000", 31, "-2147483648000000000000");
            test("-1000000000000", 32, "-4294967296000000000000");
            test("-1000000000000", 33, "-8589934592000000000000");
            test(
                "-1000000000000",
                100,
                "-1267650600228229401496703205376000000000000",
            );

            test("123", 0, "123");
            test("245", -1, "122");
            test("246", -1, "123");
            test("247", -1, "123");
            test("491", -2, "122");
            test("492", -2, "123");
            test("493", -2, "123");
            test("4127195135", -25, "122");
            test("4127195136", -25, "123");
            test("4127195137", -25, "123");
            test("8254390271", -26, "122");
            test("8254390272", -26, "123");
            test("8254390273", -26, "123");
            test("155921023828072216384094494261247", -100, "122");
            test("155921023828072216384094494261248", -100, "123");
            test("155921023828072216384094494261249", -100, "123");
            test("4294967295", -1, "2147483647");
            test("4294967296", -1, "2147483648");
            test("4294967297", -1, "2147483648");
            test("1000000000000", 0, "1000000000000");
            test("7999999999999", -3, "999999999999");
            test("8000000000000", -3, "1000000000000");
            test("8000000000001", -3, "1000000000000");
            test("16777216000000000000", -24, "1000000000000");
            test("33554432000000000000", -25, "1000000000000");
            test("2147483648000000000000", -31, "1000000000000");
            test("4294967296000000000000", -32, "1000000000000");
            test("8589934592000000000000", -33, "1000000000000");
            test(
                "1267650600228229401496703205376000000000000",
                -100,
                "1000000000000",
            );
            test("1000000000000", -10, "976562500");
            test("980657949", -72, "0");
            test("4294967295", -31, "1");
            test("4294967295", -32, "0");
            test("4294967296", -32, "1");
            test("4294967296", -33, "0");

            test("-123", 0, "-123");
            test("-245", -1, "-123");
            test("-246", -1, "-123");
            test("-247", -1, "-124");
            test("-491", -2, "-123");
            test("-492", -2, "-123");
            test("-493", -2, "-124");
            test("-4127195135", -25, "-123");
            test("-4127195136", -25, "-123");
            test("-4127195137", -25, "-124");
            test("-8254390271", -26, "-123");
            test("-8254390272", -26, "-123");
            test("-8254390273", -26, "-124");
            test("-155921023828072216384094494261247", -100, "-123");
            test("-155921023828072216384094494261248", -100, "-123");
            test("-155921023828072216384094494261249", -100, "-124");
            test("-4294967295", -1, "-2147483648");
            test("-4294967296", -1, "-2147483648");
            test("-4294967297", -1, "-2147483649");
            test("-1000000000000", 0, "-1000000000000");
            test("-7999999999999", -3, "-1000000000000");
            test("-8000000000000", -3, "-1000000000000");
            test("-8000000000001", -3, "-1000000000001");
            test("-16777216000000000000", -24, "-1000000000000");
            test("-33554432000000000000", -25, "-1000000000000");
            test("-2147483648000000000000", -31, "-1000000000000");
            test("-4294967296000000000000", -32, "-1000000000000");
            test("-8589934592000000000000", -33, "-1000000000000");
            test(
                "-1267650600228229401496703205376000000000000",
                -100,
                "-1000000000000",
            );
            test("-1000000000000", -10, "-976562500");
            test("-980657949", -72, "-1");
            test("-4294967295", -31, "-2");
            test("-4294967295", -32, "-1");
            test("-4294967296", -32, "-1");
            test("-4294967296", -33, "-1");
        }
    };
}
tests_signed!(i8, test_shl_i8, i, j, out, {}, n);
tests_signed!(i16, test_shl_i16, i, j, out, {}, n);
tests_signed!(
    i32,
    test_shl_i32,
    i,
    j,
    out,
    {
        let mut n = rug::Integer::from_str(i).unwrap();
        n <<= j;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(i).unwrap() << j;
        assert_eq!(n.to_string(), out);
    },
    n
);
tests_signed!(i64, test_shl_i64, i, j, out, {}, n);
tests_signed!(i128, test_shl_i128, i, j, out, {}, n);
tests_signed!(isize, test_shl_isize, i, j, out, {}, n);

fn shl_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Integer: Shl<T, Output = Integer> + ShlAssign<T> + Shr<T, Output = Integer>,
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
    u64: CheckedFrom<T>,
{
    integer_unsigned_pair_gen_var_2::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n <<= u;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() << u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert!((&n << u).abs() >= (&n).abs());
        assert_eq!(-&n << u, -(&n << u));

        assert_eq!(&n << u, &n * Integer::power_of_2(u64::exact_from(u)));
        assert_eq!(&n << u >> u, n);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    unsigned_gen_var_5::<T>().test_properties(|u| {
        assert_eq!(Integer::ZERO << u, 0);
        assert!(Natural::exact_from(Integer::ONE << u).is_power_of_2());
    });

    natural_unsigned_pair_gen_var_4::<T>().test_properties(|(n, u)| {
        assert_eq!(&n << u, Integer::from(n) << u);
    });
}

fn shl_properties_helper_signed<T: PrimitiveSigned>()
where
    Integer: Shl<T, Output = Integer> + ShlAssign<T>,
    for<'a> &'a Integer: Shl<T, Output = Integer>
        + Shl<<T as UnsignedAbs>::Output, Output = Integer>
        + ShlRound<T, Output = Integer>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    integer_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n <<= i;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n).shl_round(i, RoundingMode::Floor), shifted);

        if i >= T::ZERO {
            assert_eq!(&n << i.unsigned_abs(), shifted);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(Integer::ZERO << i, 0);
    });

    natural_signed_pair_gen_var_2::<T>().test_properties(|(n, i)| {
        assert_eq!(&n << i, Integer::from(n) << i);
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_properties_helper_signed);

    integer_unsigned_pair_gen_var_2::<u32>().test_properties(|(n, u)| {
        let shifted = &n << u;
        let mut rug_n = integer_to_rug_integer(&n);
        rug_n <<= u;
        assert_eq!(rug_integer_to_integer(&rug_n), shifted);
        assert_eq!(
            bigint_to_integer(&(&integer_to_bigint(&n) << usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(&n) << usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(&n) << u)),
            shifted
        );
    });

    integer_signed_pair_gen_var_1::<i32>().test_properties(|(n, i)| {
        let shifted = &n << i;
        let mut rug_n = integer_to_rug_integer(&n);
        rug_n <<= i;
        assert_eq!(rug_integer_to_integer(&rug_n), shifted);
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(&n) << i)),
            shifted
        );
    });
}
