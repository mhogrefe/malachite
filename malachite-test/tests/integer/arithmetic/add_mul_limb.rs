use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_unsigned, pairs_of_integers,
    triples_of_integer_integer_and_unsigned,
};

#[test]
fn test_add_mul_limb() {
    let test = |u, v, c: Limb, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out, "{} {} {}", u, v, c);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("-123", "0", 5, "-123");
    test("123", "5", 1, "128");
    test("-123", "5", 1, "-118");
    test("123", "5", 100, "623");
    test("-123", "5", 100, "377");
    test("10", "3", 4, "22");
    test("10", "-3", 4, "-2");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 0x1_0000, "1004294967296");
    test("-1000000000000", "-65536", 0x1_0000, "-1004294967296");
    test("-1000000000000", "65536", 0x1_0000, "-995705032704");
    test("1000000000000", "-65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
    test("-1", "1000000000000", 100, "99999999999999");
    test("0", "-1000000000000", 100, "-100000000000000");
    test("1", "-1000000000000", 100, "-99999999999999");
    test("1000000000000", "-1000000000000", 1, "0");
    test("-1000000000000", "1000000000000", 1, "0");
    test(
        "1000000000000000000000",
        "-1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "-1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "-7671776751",
        "2017407396563853588311730593327576",
        2553057608,
        "5150557302232819461437363666941314271221457",
    );
}

#[test]
fn add_mul_limb_properties() {
    test_properties(
        triples_of_integer_integer_and_unsigned,
        |&(ref a, ref b, c): &(Integer, Integer, Limb)| {
            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b, c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result, "{} {} {}", a, b, c);

            let result_alt = a.add_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.clone().add_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a + b * c, result);
            assert_eq!((-a).add_mul(-b, c), -&result);
            assert_eq!(a.add_mul(b, &Integer::from(c)), result);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.add_mul(-n, 1 as Limb), 0 as Limb);
    });

    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, c): &(Integer, Limb)| {
            assert_eq!(n.add_mul(&Integer::ZERO, c), *n);
            assert_eq!(n.add_mul(&Integer::ONE, c), n + c);
            assert_eq!(n.add_mul(&Integer::NEGATIVE_ONE, c), n - c);
            assert_eq!(Integer::ZERO.add_mul(n, c), n * c);
            assert_eq!((n * c).add_mul(-n, c), 0 as Limb);
        },
    );

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(b, 0 as Limb), *a);
        assert_eq!(a.add_mul(b, 1 as Limb), a + b);
    });
}
