use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_q::test_util::generators::{
    rational_rational_signed_triple_gen, rational_rational_unsigned_triple_gen,
    rational_signed_pair_gen, rational_signed_signed_triple_gen, rational_unsigned_pair_gen,
    rational_unsigned_unsigned_triple_gen,
};
use malachite_q::Rational;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_u32() {
    let test = |s, v: u32, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("-123", 123, Some(Ordering::Less));
    test("123", 124, Some(Ordering::Less));
    test("-123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));

    test("99/100", 1, Some(Ordering::Less));
    test("101/100", 1, Some(Ordering::Greater));
    test("22/7", 3, Some(Ordering::Greater));
    test("22/7", 4, Some(Ordering::Less));
    test("-99/100", 1, Some(Ordering::Less));
    test("-101/100", 1, Some(Ordering::Less));
    test("-22/7", 3, Some(Ordering::Less));
    test("-22/7", 4, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_u64() {
    let test = |s, v: u64, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("-123", 123, Some(Ordering::Less));
    test("123", 124, Some(Ordering::Less));
    test("-123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("-1000000000000", 1000000000000, Some(Ordering::Less));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("-1000000000000", 1000000000001, Some(Ordering::Less));

    test("99/100", 1, Some(Ordering::Less));
    test("101/100", 1, Some(Ordering::Greater));
    test("22/7", 3, Some(Ordering::Greater));
    test("22/7", 4, Some(Ordering::Less));
    test("-99/100", 1, Some(Ordering::Less));
    test("-101/100", 1, Some(Ordering::Less));
    test("-22/7", 3, Some(Ordering::Less));
    test("-22/7", 4, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Rational::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("-123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("-123", 124, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("-123", -122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));

    test("99/100", 1, Some(Ordering::Less));
    test("101/100", 1, Some(Ordering::Greater));
    test("22/7", 3, Some(Ordering::Greater));
    test("22/7", 4, Some(Ordering::Less));
    test("-99/100", -1, Some(Ordering::Greater));
    test("-101/100", -1, Some(Ordering::Less));
    test("-22/7", -3, Some(Ordering::Less));
    test("-22/7", -4, Some(Ordering::Greater));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Rational::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("-123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("-123", 124, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("-123", -122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", -1000000000000, Some(Ordering::Greater));
    test("-1000000000000", 1000000000000, Some(Ordering::Less));
    test("-1000000000000", -1000000000000, Some(Ordering::Equal));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("1000000000000", -1000000000001, Some(Ordering::Greater));
    test("-1000000000000", 1000000000001, Some(Ordering::Less));
    test("-1000000000000", -1000000000001, Some(Ordering::Greater));

    test("99/100", 1, Some(Ordering::Less));
    test("101/100", 1, Some(Ordering::Greater));
    test("22/7", 3, Some(Ordering::Greater));
    test("22/7", 4, Some(Ordering::Less));
    test("-99/100", -1, Some(Ordering::Greater));
    test("-101/100", -1, Some(Ordering::Less));
    test("-22/7", -3, Some(Ordering::Less));
    test("-22/7", -4, Some(Ordering::Greater));
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_unsigned<
    T: PartialOrd<Rational> + PartialOrd<rug::Rational> + PrimitiveUnsigned,
>()
where
    Rational: From<T> + PartialOrd<T>,
    rug::Rational: PartialOrd<T>,
{
    rational_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(rug::Rational::from(&n).partial_cmp(&u), cmp);
        assert_eq!(Some(n.cmp(&Rational::from(u))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rug::Rational::from(&n)), cmp_rev);
        assert_eq!(Some(Rational::from(u).cmp(&n)), cmp_rev);
    });

    rational_rational_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > u && u > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    rational_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Rational::from(y)), Some(x.cmp(&y)));
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_signed<
    T: PartialOrd<Rational> + PartialOrd<rug::Rational> + PrimitiveSigned,
>()
where
    Rational: From<T> + PartialOrd<T>,
    rug::Rational: PartialOrd<T>,
{
    rational_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(rug::Rational::from(&n).partial_cmp(&i), cmp);
        assert_eq!(Some(n.cmp(&Rational::from(i))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(i.partial_cmp(&n), cmp_rev);
        assert_eq!(i.partial_cmp(&rug::Rational::from(&n)), cmp_rev);
        assert_eq!(Some(Rational::from(i).cmp(&n)), cmp_rev);
    });

    rational_rational_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n < i && i < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > i && i > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    rational_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Rational::from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_primitive_int_properties_helper_signed);
}
