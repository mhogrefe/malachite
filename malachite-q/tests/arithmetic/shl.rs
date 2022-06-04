use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen_var_5};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_1;
use malachite_q::test_util::common::{rational_to_rug_rational, rug_rational_to_rational};
use malachite_q::test_util::generators::{
    rational_gen, rational_signed_pair_gen_var_1, rational_unsigned_pair_gen_var_1,
};
use malachite_q::Rational;
use std::ops::{Shl, ShlAssign, Shr};
use std::str::FromStr;

fn test_shl_unsigned_helper<T: PrimitiveUnsigned, F: Fn(&str, T, &str)>(f: F)
where
    Rational: ShlAssign<T> + Shl<T, Output = Rational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
{
    let test = |s, v: u8, out| {
        let u = Rational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");

    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");

    test("-123", 0, "-123");
    test("-123", 1, "-246");
    test("-123", 2, "-492");
    test("-123", 25, "-4127195136");
    test("-123", 26, "-8254390272");
    test("-123", 100, "-155921023828072216384094494261248");

    test("22/7", 0, "22/7");
    test("22/7", 1, "44/7");
    test("22/7", 2, "88/7");
    test("22/7", 25, "738197504/7");
    test("22/7", 26, "1476395008/7");
    test("22/7", 100, "27888313205021046832927470518272/7");

    test("-22/7", 0, "-22/7");
    test("-22/7", 1, "-44/7");
    test("-22/7", 2, "-88/7");
    test("-22/7", 25, "-738197504/7");
    test("-22/7", 26, "-1476395008/7");
    test("-22/7", 100, "-27888313205021046832927470518272/7");
}

#[test]
fn test_shl_unsigned() {
    test_shl_unsigned_helper::<u8, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u16, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u32, _>(|s, v, out| {
        let mut n = rug::Rational::from_str(s).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() << v;
        assert_eq!(n.to_string(), out);
    });
    test_shl_unsigned_helper::<u64, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u128, _>(|_, _, _| {});
    test_shl_unsigned_helper::<usize, _>(|_, _, _| {});
}

fn test_shl_signed_helper<T: PrimitiveSigned, F: Fn(&str, T, &str)>(f: F)
where
    Rational: ShlAssign<T> + Shl<T, Output = Rational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
{
    let test = |s, v: i8, out| {
        let u = Rational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("0", 10, "0");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");

    test("-123", 1, "-246");
    test("-123", 2, "-492");
    test("-123", 25, "-4127195136");
    test("-123", 26, "-8254390272");
    test("-123", 100, "-155921023828072216384094494261248");

    test("123", 0, "123");
    test("245", -1, "245/2");
    test("246", -1, "123");
    test("247", -1, "247/2");
    test("491", -2, "491/4");
    test("492", -2, "123");
    test("493", -2, "493/4");

    test("-123", 0, "-123");
    test("-245", -1, "-245/2");
    test("-246", -1, "-123");
    test("-247", -1, "-247/2");
    test("-491", -2, "-491/4");
    test("-492", -2, "-123");
    test("-493", -2, "-493/4");

    test("22/7", 0, "22/7");
    test("22/7", 1, "44/7");
    test("22/7", 2, "88/7");
    test("22/7", 25, "738197504/7");
    test("22/7", 26, "1476395008/7");
    test("22/7", 100, "27888313205021046832927470518272/7");

    test("-22/7", 0, "-22/7");
    test("-22/7", 1, "-44/7");
    test("-22/7", 2, "-88/7");
    test("-22/7", 25, "-738197504/7");
    test("-22/7", 26, "-1476395008/7");
    test("-22/7", 100, "-27888313205021046832927470518272/7");

    test("22/7", -1, "11/7");
    test("22/7", -2, "11/14");
    test("22/7", -25, "11/117440512");
    test("22/7", -26, "11/234881024");
    test("22/7", -100, "11/4436777100798802905238461218816");

    test("-22/7", -1, "-11/7");
    test("-22/7", -2, "-11/14");
    test("-22/7", -25, "-11/117440512");
    test("-22/7", -26, "-11/234881024");
    test("-22/7", -100, "-11/4436777100798802905238461218816");
}

#[test]
fn test_shl_signed() {
    test_shl_signed_helper::<i8, _>(|_, _, _| {});
    test_shl_signed_helper::<i16, _>(|_, _, _| {});
    test_shl_signed_helper::<i32, _>(|s, v, out| {
        let mut n = rug::Rational::from_str(s).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() << v;
        assert_eq!(n.to_string(), out);
    });
    test_shl_signed_helper::<i64, _>(|_, _, _| {});
    test_shl_signed_helper::<i128, _>(|_, _, _| {});
    test_shl_signed_helper::<isize, _>(|_, _, _| {});
}

fn shl_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Rational: Shl<T, Output = Rational> + ShlAssign<T> + Shr<T, Output = Rational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
    u64: CheckedFrom<T>,
{
    rational_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
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

        assert!((&n << u).ge_abs(&n));
        assert_eq!(-&n << u, -(&n << u));

        assert_eq!(&n << u, &n * Rational::power_of_2(u64::exact_from(u)));
        assert_eq!(&n << u >> u, n);
    });

    rational_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    unsigned_gen_var_5::<T>().test_properties(|u| {
        assert_eq!(Rational::ZERO << u, 0);
        assert!((Rational::ONE << u).is_power_of_2());
    });

    integer_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        assert_eq!(&n << u, Rational::from(n) << u);
    });
}

fn shl_properties_helper_signed<T: PrimitiveSigned>()
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Rational: Shl<T, Output = Rational> + ShlAssign<T> + Shr<T, Output = Rational>,
    for<'a> &'a Rational:
        Shl<T, Output = Rational> + Shl<<T as UnsignedAbs>::Output, Output = Rational>,
    i64: CheckedFrom<T>,
{
    rational_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
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

        if i >= T::ZERO {
            assert_eq!(&n << i.unsigned_abs(), shifted);
        }
        assert_eq!(-&n << i, -(&n << i));

        assert_eq!(&n << i, &n * Rational::power_of_2(i64::exact_from(i)));
        assert_eq!(&n << i >> i, n);
        if let Some(neg_i) = i.checked_neg() {
            assert_eq!(&n << neg_i, n >> i);
        }
    });

    rational_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(Rational::ZERO << i, 0);
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_properties_helper_signed);

    rational_unsigned_pair_gen_var_1::<u32>().test_properties(|(n, u)| {
        let shifted = &n << u;
        let mut rug_n = rational_to_rug_rational(&n);
        rug_n <<= u;
        assert_eq!(rug_rational_to_rational(&rug_n), shifted);
        assert_eq!(
            rug_rational_to_rational(&(rational_to_rug_rational(&n) << u)),
            shifted
        );
    });

    rational_signed_pair_gen_var_1::<i32>().test_properties(|(n, i)| {
        let shifted = &n << i;
        let mut rug_n = rational_to_rug_rational(&n);
        rug_n <<= i;
        assert_eq!(rug_rational_to_rational(&rug_n), shifted);
        assert_eq!(
            rug_rational_to_rational(&(rational_to_rug_rational(&n) << i)),
            shifted
        );
    });
}