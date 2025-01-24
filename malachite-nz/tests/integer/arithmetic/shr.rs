// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShrRound, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_1, unsigned_gen,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_signed_pair_gen_var_1, integer_unsigned_pair_gen_var_2,
    integer_unsigned_unsigned_triple_gen_var_3, natural_signed_pair_gen_var_2,
    natural_unsigned_pair_gen_var_4,
};
use num::BigInt;
use std::ops::{Shr, ShrAssign};
use std::str::FromStr;

fn test_shr_unsigned_helper<T: PrimitiveUnsigned, F: Fn(&str, T, &str)>(f: F)
where
    Integer: Shr<T, Output = Integer> + ShrAssign<T>,
    for<'a> &'a Integer: Shr<T, Output = Integer>,
{
    let test = |s, v: u8, out| {
        let u = Integer::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");

    test("123", 0, "123");
    test("245", 1, "122");
    test("246", 1, "123");
    test("247", 1, "123");
    test("491", 2, "122");
    test("492", 2, "123");
    test("493", 2, "123");
    test("4127195135", 25, "122");
    test("4127195136", 25, "123");
    test("4127195137", 25, "123");
    test("8254390271", 26, "122");
    test("8254390272", 26, "123");
    test("8254390273", 26, "123");
    test("155921023828072216384094494261247", 100, "122");
    test("155921023828072216384094494261248", 100, "123");
    test("155921023828072216384094494261249", 100, "123");
    test("4294967295", 1, "2147483647");
    test("4294967296", 1, "2147483648");
    test("4294967297", 1, "2147483648");
    test("1000000000000", 0, "1000000000000");
    test("7999999999999", 3, "999999999999");
    test("8000000000000", 3, "1000000000000");
    test("8000000000001", 3, "1000000000000");
    test("16777216000000000000", 24, "1000000000000");
    test("33554432000000000000", 25, "1000000000000");
    test("2147483648000000000000", 31, "1000000000000");
    test("4294967296000000000000", 32, "1000000000000");
    test("8589934592000000000000", 33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        100,
        "1000000000000",
    );
    test("1000000000000", 10, "976562500");
    test("980657949", 72, "0");
    test("4294967295", 31, "1");
    test("4294967295", 32, "0");
    test("4294967296", 32, "1");
    test("4294967296", 33, "0");

    test("-123", 0, "-123");
    test("-245", 1, "-123");
    test("-246", 1, "-123");
    test("-247", 1, "-124");
    test("-491", 2, "-123");
    test("-492", 2, "-123");
    test("-493", 2, "-124");
    test("-4127195135", 25, "-123");
    test("-4127195136", 25, "-123");
    test("-4127195137", 25, "-124");
    test("-8254390271", 26, "-123");
    test("-8254390272", 26, "-123");
    test("-8254390273", 26, "-124");
    test("-155921023828072216384094494261247", 100, "-123");
    test("-155921023828072216384094494261248", 100, "-123");
    test("-155921023828072216384094494261249", 100, "-124");
    test("-4294967295", 1, "-2147483648");
    test("-4294967296", 1, "-2147483648");
    test("-4294967297", 1, "-2147483649");
    test("-1000000000000", 0, "-1000000000000");
    test("-7999999999999", 3, "-1000000000000");
    test("-8000000000000", 3, "-1000000000000");
    test("-8000000000001", 3, "-1000000000001");
    test("-16777216000000000000", 24, "-1000000000000");
    test("-33554432000000000000", 25, "-1000000000000");
    test("-2147483648000000000000", 31, "-1000000000000");
    test("-4294967296000000000000", 32, "-1000000000000");
    test("-8589934592000000000000", 33, "-1000000000000");
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        "-1000000000000",
    );
    test("-1000000000000", 10, "-976562500");
    test("-980657949", 72, "-1");
    test("-4294967295", 31, "-2");
    test("-4294967295", 32, "-1");
    test("-4294967296", 32, "-1");
    test("-4294967296", 33, "-1");
}

#[test]
fn test_shr_unsigned() {
    test_shr_unsigned_helper::<u8, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u16, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u32, _>(|u, v, out| {
        let mut n = rug::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
    });
    test_shr_unsigned_helper::<u64, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u128, _>(|_, _, _| {});
    test_shr_unsigned_helper::<usize, _>(|_, _, _| {});
}

fn test_shr_signed_helper<T: PrimitiveSigned, F: Fn(&str, T, &str)>(f: F)
where
    Integer: Shr<T, Output = Integer> + ShrAssign<T>,
    for<'a> &'a Integer: Shr<T, Output = Integer>,
{
    let test = |s, v: i8, out| {
        let u = Integer::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");

    test("123", 0, "123");
    test("245", 1, "122");
    test("246", 1, "123");
    test("247", 1, "123");
    test("491", 2, "122");
    test("492", 2, "123");
    test("493", 2, "123");
    test("4127195135", 25, "122");
    test("4127195136", 25, "123");
    test("4127195137", 25, "123");
    test("8254390271", 26, "122");
    test("8254390272", 26, "123");
    test("8254390273", 26, "123");
    test("155921023828072216384094494261247", 100, "122");
    test("155921023828072216384094494261248", 100, "123");
    test("155921023828072216384094494261249", 100, "123");
    test("4294967295", 1, "2147483647");
    test("4294967296", 1, "2147483648");
    test("4294967297", 1, "2147483648");
    test("1000000000000", 0, "1000000000000");
    test("7999999999999", 3, "999999999999");
    test("8000000000000", 3, "1000000000000");
    test("8000000000001", 3, "1000000000000");
    test("16777216000000000000", 24, "1000000000000");
    test("33554432000000000000", 25, "1000000000000");
    test("2147483648000000000000", 31, "1000000000000");
    test("4294967296000000000000", 32, "1000000000000");
    test("8589934592000000000000", 33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        100,
        "1000000000000",
    );
    test("1000000000000", 10, "976562500");
    test("980657949", 72, "0");
    test("4294967295", 31, "1");
    test("4294967295", 32, "0");
    test("4294967296", 32, "1");
    test("4294967296", 33, "0");

    test("-123", 0, "-123");
    test("-245", 1, "-123");
    test("-246", 1, "-123");
    test("-247", 1, "-124");
    test("-491", 2, "-123");
    test("-492", 2, "-123");
    test("-493", 2, "-124");
    test("-4127195135", 25, "-123");
    test("-4127195136", 25, "-123");
    test("-4127195137", 25, "-124");
    test("-8254390271", 26, "-123");
    test("-8254390272", 26, "-123");
    test("-8254390273", 26, "-124");
    test("-155921023828072216384094494261247", 100, "-123");
    test("-155921023828072216384094494261248", 100, "-123");
    test("-155921023828072216384094494261249", 100, "-124");
    test("-4294967295", 1, "-2147483648");
    test("-4294967296", 1, "-2147483648");
    test("-4294967297", 1, "-2147483649");
    test("-1000000000000", 0, "-1000000000000");
    test("-7999999999999", 3, "-1000000000000");
    test("-8000000000000", 3, "-1000000000000");
    test("-8000000000001", 3, "-1000000000001");
    test("-16777216000000000000", 24, "-1000000000000");
    test("-33554432000000000000", 25, "-1000000000000");
    test("-2147483648000000000000", 31, "-1000000000000");
    test("-4294967296000000000000", 32, "-1000000000000");
    test("-8589934592000000000000", 33, "-1000000000000");
    test(
        "-1267650600228229401496703205376000000000000",
        100,
        "-1000000000000",
    );
    test("-1000000000000", 10, "-976562500");
    test("-980657949", 72, "-1");
    test("-4294967295", 31, "-2");
    test("-4294967295", 32, "-1");
    test("-4294967296", 32, "-1");
    test("-4294967296", 33, "-1");

    test("0", -10, "0");
    test("123", -1, "246");
    test("123", -2, "492");
    test("123", -25, "4127195136");
    test("123", -26, "8254390272");
    test("123", -100, "155921023828072216384094494261248");
    test("2147483648", -1, "4294967296");
    test("1000000000000", -3, "8000000000000");
    test("1000000000000", -24, "16777216000000000000");
    test("1000000000000", -25, "33554432000000000000");
    test("1000000000000", -31, "2147483648000000000000");
    test("1000000000000", -32, "4294967296000000000000");
    test("1000000000000", -33, "8589934592000000000000");
    test(
        "1000000000000",
        -100,
        "1267650600228229401496703205376000000000000",
    );

    test("-123", -1, "-246");
    test("-123", -2, "-492");
    test("-123", -25, "-4127195136");
    test("-123", -26, "-8254390272");
    test("-123", -100, "-155921023828072216384094494261248");
    test("-2147483648", -1, "-4294967296");
    test("-1000000000000", -3, "-8000000000000");
    test("-1000000000000", -24, "-16777216000000000000");
    test("-1000000000000", -25, "-33554432000000000000");
    test("-1000000000000", -31, "-2147483648000000000000");
    test("-1000000000000", -32, "-4294967296000000000000");
    test("-1000000000000", -33, "-8589934592000000000000");
    test(
        "-1000000000000",
        -100,
        "-1267650600228229401496703205376000000000000",
    );
}

#[test]
fn test_shr_signed() {
    test_shr_signed_helper::<i8, _>(|_, _, _| {});
    test_shr_signed_helper::<i16, _>(|_, _, _| {});
    test_shr_signed_helper::<i32, _>(|u, v, out| {
        let mut n = rug::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
    });
    test_shr_signed_helper::<i64, _>(|_, _, _| {});
    test_shr_signed_helper::<i128, _>(|_, _, _| {});
    test_shr_signed_helper::<isize, _>(|_, _, _| {});
}

fn shr_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    Integer: ShrAssign<T> + Shr<T, Output = Integer>,
    for<'a> &'a Integer: Shr<T, Output = Integer> + ShrRound<T, Output = Integer>,
    for<'a> &'a Natural: Shr<T, Output = Natural>,
    SignedLimb: Shr<T, Output = SignedLimb>,
{
    integer_unsigned_pair_gen_var_2::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n >>= u;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() >> u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert!(shifted.le_abs(&n));
        assert_eq!((&n).shr_round(u, Floor).0, shifted);
    });

    integer_unsigned_unsigned_triple_gen_var_3::<T>().test_properties(|(n, u, v)| {
        if let Some(sum) = u.checked_add(v) {
            assert_eq!(&n >> u >> v, n >> sum);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!(&n >> T::ZERO, n);
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(Integer::ZERO >> u, 0);
    });

    natural_unsigned_pair_gen_var_4::<T>().test_properties(|(n, u)| {
        assert_eq!(&n >> u, Integer::from(n) >> u);
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, T>().test_properties(|(i, j)| {
        if let Some(sum) = j.checked_add(T::exact_from(SignedLimb::WIDTH)) {
            let shifted = Integer::from(i) >> sum;
            if i >= 0 {
                assert_eq!(shifted, 0);
            } else {
                assert_eq!(shifted, -1);
            }
        }
        if j < T::exact_from(SignedLimb::WIDTH) {
            assert_eq!(i >> j, Integer::from(i) >> j);
        }
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn shr_properties_helper_signed<T: PrimitiveSigned>()
where
    Integer:
        ShrAssign<T> + Shr<T, Output = Integer> + Shr<<T as UnsignedAbs>::Output, Output = Integer>,
    for<'a> &'a Integer: Shr<T, Output = Integer> + ShrRound<T, Output = Integer>,
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    integer_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n >>= i;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> i;
        assert_eq!(shifted_alt, shifted);
        assert!(shifted_alt.is_valid());
        let shifted_alt = n.clone() >> i;
        assert_eq!(shifted_alt, shifted);
        assert!(shifted_alt.is_valid());

        assert_eq!((&n).shr_round(i, Floor).0, shifted);

        if i >= T::ZERO {
            assert_eq!(n >> i.unsigned_abs(), shifted);
        }
    });

    integer_gen().test_properties(|n| {
        assert_eq!(&n >> T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(Integer::ZERO >> i, 0);
    });

    natural_signed_pair_gen_var_2::<T>().test_properties(|(n, i)| {
        assert_eq!(&n >> i, Integer::from(n) >> i);
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds!(shr_properties_helper_unsigned);
    apply_fn_to_signeds!(shr_properties_helper_signed);

    integer_unsigned_pair_gen_var_2::<u32>().test_properties(|(n, u)| {
        let shifted = &n >> u;
        let mut rug_n = rug::Integer::from(&n);
        rug_n >>= u;
        assert_eq!(Integer::from(&rug_n), shifted);

        assert_eq!(Integer::from(&(rug::Integer::from(&n) >> u)), shifted);

        assert_eq!(
            Integer::from(&(&BigInt::from(&n) >> usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            Integer::from(&(BigInt::from(&n) >> usize::exact_from(u))),
            shifted
        );
    });

    integer_signed_pair_gen_var_1::<i32>().test_properties(|(n, i)| {
        let shifted = &n >> i;
        let mut rug_n = rug::Integer::from(&n);
        rug_n >>= i;
        assert_eq!(Integer::from(&rug_n), shifted);

        assert_eq!(Integer::from(&(rug::Integer::from(&n) >> i)), shifted);
    });
}
