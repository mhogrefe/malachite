use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigInt;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{natural_signeds, signeds, unsigneds};

macro_rules! unsigned_properties {
    ($u: ident) => {
        test_properties(unsigneds::<$u>, |&u| {
            let n = Integer::from(u);
            assert!(n.is_valid());
            assert_eq!($u::exact_from(&n), u);
            assert_eq!(Integer::from(Natural::from(u)), n);
            assert_eq!(Integer::from(u128::exact_from(u)), n);
        });
    };
}

macro_rules! signed_properties {
    ($s: ident) => {
        test_properties(signeds::<$s>, |&i| {
            let n = Integer::from(i);
            assert!(n.is_valid());
            assert_eq!($s::exact_from(&n), i);
            assert_eq!(Integer::from(i128::exact_from(i)), n);
        });

        test_properties(natural_signeds::<$s>, |&i| {
            assert_eq!(Integer::from(Natural::exact_from(i)), Integer::from(i));
        });
    };
}

#[test]
fn from_primitive_integer_properties() {
    test_properties(unsigneds::<u32>, |&u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    test_properties(unsigneds::<u64>, |&u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    test_properties(signeds::<i32>, |&i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });

    test_properties(signeds::<i64>, |&i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });

    unsigned_properties!(u8);
    unsigned_properties!(u16);
    unsigned_properties!(u32);
    unsigned_properties!(u64);
    unsigned_properties!(usize);

    signed_properties!(i8);
    signed_properties!(i16);
    signed_properties!(i32);
    signed_properties!(i64);
    signed_properties!(isize);
}
