use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_base::num::{AddMul, AddMulAssign};
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_naturals,
                                      triples_of_natural_natural_and_unsigned};
use std::str::FromStr;

#[test]
fn test_add_mul_u32() {
    let test = |u, v, c: u32, out| {
        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u)
            .unwrap()
            .add_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u)
            .unwrap()
            .add_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Natural::from_str(u).unwrap()).add_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Natural::from_str(u).unwrap()).add_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "128");
    test("123", "5", 100, "623");
    test("10", "3", 4, "22");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 0x1_0000, "1004294967296");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
}

#[test]
fn add_mul_u32_properties() {
    // a.add_mul_assign(b, c); a is valid.
    // a.add_mul_assign(&b, c); a is valid.
    // a.add_mul(b, c) is valid.
    // a.add_mul(&b, c) is valid.
    // (&a).add_mul(b, c) is valid.
    // (&a).add_mul(&b, c) is valid.
    // a.add_mul_assign(b, c), a.add_mul_assign(&b, c), a.add_mul(b, c), a.add_mul(&b, c),
    //      (&a).add_mul(b, c), and (&a).add_mul(&b, c) give the same result.
    // a.add_mul(&b, c) is equivalent to a + b * c.
    // a.add_mul(&b, c) is equivalent to a.add_mul(&b, Natural::from(c))
    let natural_natural_and_u32 = |mut a: Natural, b: Natural, c: u32| {
        let old_a = a.clone();
        a.add_mul_assign(b.clone(), c);
        assert!(a.is_valid());

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let a2 = old_a.clone();
        let result = (&a2).add_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let a2 = old_a.clone();
        let result = (&a2).add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a + &b * c, result);
        assert_eq!(old_a.add_mul(b, Natural::from(c)), result);
    };

    // n.add_mul(0, c) == n
    // n.add_mul(1, c) == n + c
    // 0.add_mul(n, c) == n * c
    let natural_and_u32 = |n: &Natural, c: u32| {
        assert_eq!(n.add_mul(&Natural::ZERO, c), *n);
        assert_eq!(n.add_mul(&Natural::ONE, c), n + c);
        assert_eq!(Natural::ZERO.add_mul(n, c), n * c);
    };

    // a.add_mul(b, 0) == a
    // a.add_mul(b, 1) == a + b
    let two_naturals = |a: &Natural, b: &Natural| {
        assert_eq!(a.add_mul(b, 0), *a);
        assert_eq!(a.add_mul(b, 1), a + b);
    };

    for (a, b, c) in
        triples_of_natural_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for (a, b, c) in
        triples_of_natural_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for (n, c) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(&n, c);
    }

    for (n, c) in pairs_of_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(&n, c);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }
}
