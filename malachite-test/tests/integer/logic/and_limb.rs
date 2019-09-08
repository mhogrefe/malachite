use std::str::FromStr;

use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};

use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::logic::and_limb::{integer_and_limb_alt_1, integer_and_limb_alt_2};

#[test]
fn test_and_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n, out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n &= v;
            assert_eq!(n, out);
        }

        assert_eq!(Integer::from_str(u).unwrap() & v, out);
        assert_eq!(&Integer::from_str(u).unwrap() & v, out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() & v;
            assert_eq!(n, out);
        }

        assert_eq!(v & Integer::from_str(u).unwrap(), out);
        assert_eq!(v & &Integer::from_str(u).unwrap(), out);

        assert_eq!(
            integer_and_limb_alt_1(&Integer::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(
            integer_and_limb_alt_2(&Integer::from_str(u).unwrap(), v),
            out
        );

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v & rug::Integer::from_str(u).unwrap();
            assert_eq!(n, out);
        }

        let mut n = v;
        n &= &Integer::from_str(u).unwrap();
        assert_eq!(n, out);

        let mut n = v;
        n &= Integer::from_str(u).unwrap();
        assert_eq!(n, out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(v & &rug::Integer::from_str(u).unwrap());
            assert_eq!(n, out);
        }
    };
    test("0", 0, 0);
    test("0", 123, 0);
    test("123", 0, 0);
    test("123", 456, 72);
    test("1000000000000", 123, 0);
    test("1000000000001", 123, 1);
    test("12345678987654321", 987_654_321, 579_887_281);
    test("-123", 0, 0);
    test("-123", 456, 384);
    test("-1000000000000", 123, 0);
    test("-1000000000001", 123, 123);
    test("-12345678987654321", 987_654_321, 407_767_041);
}

#[test]
fn and_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let mut mut_n = n.clone();
            mut_n &= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n &= u;
                assert_eq!(rug_integer_to_integer(&rug_n), result);
            }

            assert_eq!(n & u, result);
            assert_eq!(n.clone() & u, result);
            assert_eq!(u & n, result);
            assert_eq!(u & n.clone(), result);
            assert_eq!(integer_and_limb_alt_1(&n, u), result);
            assert_eq!(integer_and_limb_alt_2(&n, u), result);

            let mut mut_u = u;
            mut_u &= n;
            assert_eq!(mut_u, result);

            let mut mut_u = u;
            mut_u &= n.clone();
            assert_eq!(mut_u, result);

            assert_eq!(n & Integer::from(u), result);
            assert_eq!(Integer::from(u) & n, result);

            assert_eq!(&result & u, result);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) & u)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n & 0 as Limb, 0);
        assert_eq!(0 as Limb & n, 0);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(Integer::ZERO & u, 0);
        assert_eq!(u & Integer::ZERO, 0);
        assert_eq!(Integer::NEGATIVE_ONE & u, u);
        assert_eq!(u & Integer::NEGATIVE_ONE, u);
    });
}
