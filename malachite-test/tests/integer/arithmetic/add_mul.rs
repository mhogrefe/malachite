use common::LARGE_LIMIT;
use malachite_base::num::{AddMul, AddMulAssign, NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use std::str::FromStr;

#[test]
fn test_add_mul() {
    let test = |u, v, w, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(Integer::from_str(v).unwrap(), Integer::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            &Integer::from_str(v).unwrap(),
            Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(Integer::from_str(v).unwrap(), Integer::from_str(w).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            &Integer::from_str(v).unwrap(),
            Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u).unwrap().add_mul(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(
            &Integer::from_str(v).unwrap(),
            &Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "128");
    test("123", "5", "100", "623");
    test("10", "3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "1000000000123");
    test("1000000000000", "123", "1", "1000000000123");
    test("1000000000000", "123", "100", "1000000012300");
    test("1000000000000", "100", "123", "1000000012300");
    test("1000000000000", "65536", "65536", "1004294967296");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "2000000000000");
    test("1000000000000", "1000000000000", "100", "101000000000000");
    test("0", "1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "-5", "-1", "128");
    test("123", "-5", "-100", "623");
    test("10", "-3", "-4", "22");
    test("1000000000000", "-1", "-123", "1000000000123");
    test("1000000000000", "-123", "-1", "1000000000123");
    test("1000000000000", "-123", "-100", "1000000012300");
    test("1000000000000", "-100", "-123", "1000000012300");
    test("1000000000000", "-65536", "-65536", "1004294967296");
    test("1000000000000", "-1000000000000", "-1", "2000000000000");
    test("1000000000000", "-1000000000000", "-100", "101000000000000");
    test("0", "-1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "-5", "1", "118");
    test("123", "5", "-1", "118");
    test("123", "-5", "100", "-377");
    test("123", "5", "-100", "-377");
    test("10", "-3", "4", "-2");
    test("10", "3", "-4", "-2");
    test("15", "-3", "4", "3");
    test("15", "3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "-1", "123", "999999999877");
    test("1000000000000", "1", "-123", "999999999877");
    test("1000000000000", "-123", "1", "999999999877");
    test("1000000000000", "123", "-1", "999999999877");
    test("1000000000000", "-123", "100", "999999987700");
    test("1000000000000", "123", "-100", "999999987700");
    test("1000000000000", "-100", "123", "999999987700");
    test("1000000000000", "100", "-123", "999999987700");
    test("1000000000000", "-65536", "65536", "995705032704");
    test("1000000000000", "65536", "-65536", "995705032704");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "0");
    test("1000000000000", "1000000000000", "-1", "0");
    test("1000000000000", "-1000000000000", "100", "-99000000000000");
    test("1000000000000", "1000000000000", "-100", "-99000000000000");
    test("0", "-1000000000000", "100", "-100000000000000");
    test("4294967296", "-1", "1", "4294967295");
    test("4294967296", "1", "-1", "4294967295");
    test("3902609153", "-88817093856604", "1", "-88813191247451");
    test("3902609153", "88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "-5", "1", "-128");
    test("-123", "-5", "100", "-623");
    test("-10", "-3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-1000000000123");
    test("-1000000000000", "-123", "1", "-1000000000123");
    test("-1000000000000", "-123", "100", "-1000000012300");
    test("-1000000000000", "-100", "123", "-1000000012300");
    test("-1000000000000", "-65536", "65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "5", "-1", "-128");
    test("-123", "5", "-100", "-623");
    test("-10", "3", "-4", "-22");
    test("-1000000000000", "1", "-123", "-1000000000123");
    test("-1000000000000", "123", "-1", "-1000000000123");
    test("-1000000000000", "123", "-100", "-1000000012300");
    test("-1000000000000", "100", "-123", "-1000000012300");
    test("-1000000000000", "65536", "-65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "5", "1", "-118");
    test("-123", "-5", "-1", "-118");
    test("-123", "5", "100", "377");
    test("-123", "-5", "-100", "377");
    test("-10", "3", "4", "2");
    test("-10", "-3", "-4", "2");
    test("-15", "3", "4", "-3");
    test("-15", "-3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "1", "123", "-999999999877");
    test("-1000000000000", "-1", "-123", "-999999999877");
    test("-1000000000000", "123", "1", "-999999999877");
    test("-1000000000000", "-123", "-1", "-999999999877");
    test("-1000000000000", "123", "100", "-999999987700");
    test("-1000000000000", "-123", "-100", "-999999987700");
    test("-1000000000000", "100", "123", "-999999987700");
    test("-1000000000000", "-100", "-123", "-999999987700");
    test("-1000000000000", "65536", "65536", "-995705032704");
    test("-1000000000000", "-65536", "-65536", "-995705032704");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "0");
    test("-1000000000000", "-1000000000000", "-1", "0");
    test("-1000000000000", "1000000000000", "100", "99000000000000");
    test("-1000000000000", "-1000000000000", "-100", "99000000000000");
    test("-4294967296", "1", "1", "-4294967295");
    test("-4294967296", "-1", "-1", "-4294967295");
    test("-3902609153", "88817093856604", "1", "88813191247451");
    test("-3902609153", "-88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "-1000000000000",
        "1000000000000",
        "0",
    );
}

#[test]
fn add_mul_properties() {
    // a.add_mul_assign(b, c); a is valid.
    // a.add_mul_assign(b, &c); a is valid.
    // a.add_mul_assign(&b, c); a is valid.
    // a.add_mul_assign(&b, &c); a is valid.
    // a.add_mul(b, c) is valid.
    // a.add_mul(b, &c) is valid.
    // a.add_mul(&b, c) is valid.
    // a.add_mul(&b, &c) is valid.
    // (&a).add_mul(&b, &c) is valid.
    // a.add_mul_assign(b, c), a.add_mul_assign(b, &c), a.add_mul_assign(&b, c),
    // a.add_mul_assign(&b, &c), a.add_mul(b, c), a.add_mul(b, &c), a.add_mul(&b, c),
    //      a.add_mul(&b, &c), and (&a).add_mul(&b, &c) give the same result.
    // a.add_mul(b, c) is equivalent to a + b * c.
    // a.add_mul(b, c) is equivalent to a.add_mul(c, b).
    // a.add_mul(b, c) = a.add_mul(-b, -c)
    // -(a.add_mul(b, c)) = (-a).add_mul(-b, c) = (-a).add_mul(b, -c)
    let three_integers = |mut a: Integer, b: Integer, c: Integer| {
        let old_a = a.clone();
        a.add_mul_assign(b.clone(), c.clone());
        assert!(a.is_valid());

        let mut a2 = old_a.clone();
        a2.add_mul_assign(b.clone(), &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, c.clone());
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let a2 = old_a.clone();
        let result = a2.clone().add_mul(b.clone(), c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(b.clone(), &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(&b, c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = (&a2).add_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a + &b * &c, result);
        assert_eq!((&old_a).add_mul(&c, &b), result);
        assert_eq!((&old_a).add_mul(&(-&b), &(-&c)), result);
        assert_eq!((-&old_a).add_mul(&(-&b), &c), -&result);
        assert_eq!((-old_a).add_mul(b, -c), -&result);
    };

    // a.add_mul(a, -1) == 0
    // a.add_mul(-a, 1) == 0
    let single_integer = |a: &Integer| {
        assert_eq!(a.add_mul(a, &Integer::NEGATIVE_ONE), 0);
        assert_eq!(a.add_mul(&(-a), &Integer::ONE), 0);
    };

    // a.add_mul(0, b) == a
    // a.add_mul(1, b) == a + b
    // 0.add_mul(a, b) == a * b
    // a.add_mul(b, 0) == a
    // a.add_mul(b, 1) == a + b
    // (a * b).add_mul(-a, b) == 0
    // (a * b).add_mul(a, -b) == 0
    let two_integers = |a: &Integer, b: &Integer| {
        assert_eq!(a.add_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.add_mul(&Integer::ONE, b), a + b);
        assert_eq!(Integer::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.add_mul(b, &Integer::ONE), a + b);
        assert_eq!((a * b).add_mul(-a, b), 0);
        assert_eq!((a * b).add_mul(a, -b), 0);
    };

    for (a, b, c) in triples_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_integers(a, b, c);
    }

    for (a, b, c) in triples_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_integers(a, b, c);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        single_integer(&n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        single_integer(&n);
    }

    for (a, b) in pairs_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(&a, &b);
    }

    for (a, b) in pairs_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(&a, &b);
    }
}
