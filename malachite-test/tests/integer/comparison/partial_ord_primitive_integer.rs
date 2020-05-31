use std::cmp::Ordering;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::integer::comparison::partial_ord_primitive_integer::*;
use num::BigInt;
use rand::Rand;
use rug;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_integer_and_unsigned,
    triples_of_integer_signed_and_integer, triples_of_integer_unsigned_and_integer,
    triples_of_signed_integer_and_signed, triples_of_unsigned_integer_and_unsigned,
};

fn partial_cmp_primitive_integer_properties_helper_unsigned<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveUnsigned + Rand,
>()
where
    Integer: From<T> + PartialOrd<T>,
    BigInt: From<T>,
    rug::Integer: PartialOrd<T>,
{
    test_properties(pairs_of_integer_and_unsigned::<T>, |&(ref n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(num_partial_cmp_primitive(&integer_to_bigint(n), u), cmp);
        assert_eq!(integer_to_rug_integer(n).partial_cmp(&u), cmp);
        assert_eq!(Some(n.cmp(&Integer::from(u))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(u.partial_cmp(n), cmp_rev);
        assert_eq!(u.partial_cmp(&integer_to_rug_integer(n)), cmp_rev);
        assert_eq!(Some(Integer::from(u).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_integer_unsigned_and_integer::<T>,
        |&(ref n, u, ref m): &(Integer, T, Integer)| {
            if *n < u && u < *m {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if *n > u && u > *m {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_unsigned_integer_and_unsigned::<T>,
        |&(u, ref n, v)| {
            if u < *n && *n < v {
                assert!(u < v);
            } else if u > *n && *n > v {
                assert!(u > v);
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Integer::from(y)), Some(x.cmp(&y)));
    });
}

fn partial_cmp_primitive_integer_properties_helper_signed<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveSigned + Rand,
>()
where
    Integer: From<T> + PartialOrd<T>,
    BigInt: From<T>,
    rug::Integer: PartialOrd<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_integer_and_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(num_partial_cmp_primitive(&integer_to_bigint(n), i), cmp);
        assert_eq!(integer_to_rug_integer(n).partial_cmp(&i), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(i.partial_cmp(n), cmp_rev);
        assert_eq!(i.partial_cmp(&integer_to_rug_integer(n)), cmp_rev);
    });

    test_properties(pairs_of_integer_and_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(Some(n.cmp(&Integer::from(i))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(Some(Integer::from(i).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_integer_signed_and_integer::<T>,
        |&(ref n, i, ref m): &(Integer, T, Integer)| {
            if *n < i && i < *m {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if *n > i && i > *m {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_signed_integer_and_signed::<T>,
        |&(i, ref n, j)| {
            if i < *n && *n < j {
                assert!(i < j);
            } else if i > *n && *n > j {
                assert!(i > j);
            }
        },
    );

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Integer::from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_integer_properties() {
    partial_cmp_primitive_integer_properties_helper_unsigned::<u8>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u16>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u32>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u64>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<usize>();
    partial_cmp_primitive_integer_properties_helper_signed::<i8>();
    partial_cmp_primitive_integer_properties_helper_signed::<i16>();
    partial_cmp_primitive_integer_properties_helper_signed::<i32>();
    partial_cmp_primitive_integer_properties_helper_signed::<i64>();
    partial_cmp_primitive_integer_properties_helper_signed::<isize>();
}
