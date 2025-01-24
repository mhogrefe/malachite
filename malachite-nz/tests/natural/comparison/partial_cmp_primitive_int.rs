// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen_var_7, unsigned_pair_gen_var_27};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_gen, natural_natural_signed_triple_gen, natural_natural_unsigned_triple_gen,
    natural_signed_pair_gen, natural_signed_pair_gen_var_1, natural_signed_signed_triple_gen,
    natural_unsigned_pair_gen, natural_unsigned_unsigned_triple_gen,
};
use malachite_nz::test_util::natural::comparison::partial_cmp_primitive_int::*;
use num::BigUint;
use rug;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Equal));
    test("0", 5, Some(Less));
    test("123", 123, Some(Equal));
    test("123", 124, Some(Less));
    test("123", 122, Some(Greater));
    test("1000000000000", 123, Some(Greater));
}

#[test]
fn test_partial_cmp_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Equal));
    test("0", 5, Some(Less));
    test("123", 123, Some(Equal));
    test("123", 124, Some(Less));
    test("123", 122, Some(Greater));
    test("1000000000000", 123, Some(Greater));
    test("1000000000000", 1000000000000, Some(Equal));
    test("1000000000000", 1000000000001, Some(Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Equal));
    test("0", 5, Some(Less));
    test("0", -5, Some(Greater));
    test("123", 123, Some(Equal));
    test("123", -123, Some(Greater));
    test("123", 124, Some(Less));
    test("123", -124, Some(Greater));
    test("123", 122, Some(Greater));
    test("123", -122, Some(Greater));
    test("1000000000000", 123, Some(Greater));
    test("1000000000000", -123, Some(Greater));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(Ordering::reverse)
        );
    };
    test("0", 0, Some(Equal));
    test("0", 5, Some(Less));
    test("0", -5, Some(Greater));
    test("123", 123, Some(Equal));
    test("123", -123, Some(Greater));
    test("123", 124, Some(Less));
    test("123", -124, Some(Greater));
    test("123", 122, Some(Greater));
    test("123", -122, Some(Greater));
    test("1000000000000", 123, Some(Greater));
    test("1000000000000", -123, Some(Greater));
    test("1000000000000", 1000000000000, Some(Equal));
    test("1000000000000", -1000000000000, Some(Greater));
    test("1000000000000", 1000000000001, Some(Less));
    test("1000000000000", -1000000000001, Some(Greater));
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_unsigned<
    T: PartialOrd<Natural> + PartialOrd<rug::Integer> + PrimitiveUnsigned,
>()
where
    BigUint: From<T>,
    Natural: From<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    natural_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(num_partial_cmp_unsigned(&From::from(&n), u), cmp);
        assert_eq!(rug::Integer::from(&n).partial_cmp(&u), cmp);
        assert_eq!(Some(n.cmp(&Natural::from(u))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rug::Integer::from(&n)), cmp_rev);
        assert_eq!(Some(Natural::from(u).cmp(&n)), cmp_rev);
    });

    natural_natural_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(n.cmp(&m), Less);
        } else if n > u && u > m {
            assert_eq!(n.cmp(&m), Greater);
        }
    });

    natural_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    natural_gen().test_properties(|x| {
        assert!(x >= T::ZERO);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Natural::from(y)), Some(x.cmp(&y)));
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_signed<
    T: PartialOrd<Natural> + PartialOrd<rug::Integer> + PrimitiveSigned,
>()
where
    Integer: From<T>,
    Natural: TryFrom<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    natural_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(rug::Integer::from(&n).partial_cmp(&i), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(i.partial_cmp(&n), cmp_rev);
        assert_eq!(i.partial_cmp(&rug::Integer::from(&n)), cmp_rev);
    });

    natural_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(Some(n.cmp(&Natural::exact_from(i))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(Some(Natural::exact_from(i).cmp(&n)), cmp_rev);
    });

    natural_natural_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n < i && i < m {
            assert_eq!(n.cmp(&m), Less);
        } else if n > i && i > m {
            assert_eq!(n.cmp(&m), Greater);
        }
    });

    natural_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    });

    natural_gen().test_properties(|x| {
        assert!(x >= T::ZERO);
    });

    signed_pair_gen_var_7::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::exact_from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Natural::exact_from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_primitive_int_properties_helper_signed);
}
