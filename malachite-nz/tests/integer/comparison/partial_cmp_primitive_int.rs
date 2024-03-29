use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_integer_signed_triple_gen, integer_integer_unsigned_triple_gen,
    integer_signed_pair_gen, integer_signed_signed_triple_gen, integer_unsigned_pair_gen,
    integer_unsigned_unsigned_triple_gen,
};
use malachite_nz::test_util::integer::comparison::partial_cmp_primitive_int::*;
use num::BigInt;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_u32() {
    let test = |s, v: u32, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap().partial_cmp(&v), out);
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
}

#[test]
fn test_partial_cmp_u64() {
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap().partial_cmp(&v), out);
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
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
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
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
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
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_unsigned<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveUnsigned,
>()
where
    BigInt: From<T>,
    Integer: From<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    integer_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(num_partial_cmp_primitive(&From::from(&n), u), cmp);
        assert_eq!(rug::Integer::from(&n).partial_cmp(&u), cmp);
        assert_eq!(Some(n.cmp(&Integer::from(u))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rug::Integer::from(&n)), cmp_rev);
        assert_eq!(Some(Integer::from(u).cmp(&n)), cmp_rev);
    });

    integer_integer_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > u && u > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    integer_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Integer::from(y)), Some(x.cmp(&y)));
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_signed<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveSigned,
>()
where
    BigInt: From<T>,
    Integer: From<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    integer_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(num_partial_cmp_primitive(&From::from(&n), i), cmp);
        assert_eq!(rug::Integer::from(&n).partial_cmp(&i), cmp);
        assert_eq!(Some(n.cmp(&Integer::from(i))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(i.partial_cmp(&n), cmp_rev);
        assert_eq!(i.partial_cmp(&rug::Integer::from(&n)), cmp_rev);
        assert_eq!(Some(Integer::from(i).cmp(&n)), cmp_rev);
    });

    integer_integer_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n < i && i < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > i && i > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    integer_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Integer::from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_primitive_int_properties_helper_signed);
}
