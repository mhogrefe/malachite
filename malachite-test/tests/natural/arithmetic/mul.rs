use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer, num_biguint_to_native_natural,
                             rugint_integer_to_native_natural};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples_from_single, random_pairs,
                                     random_pairs_from_single, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_mul() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n *= native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = native::Natural::from_str(u).unwrap();
        n *= &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n *= gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = native::Natural::from_str(u).unwrap();
        n._basecase_mul_assign_with_mem_opt(native::Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n *= &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap() * native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Natural::from_str(u).unwrap() * native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap() * &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Natural::from_str(u).unwrap() * &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() * gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() * &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() * gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() * &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigUint::from_str(u).unwrap() * num::BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() * rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("1", "123", "123");
    test("123", "1", "123");
    test("123", "456", "56088");
    test("0", "1000000000000", "0");
    test("1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
}

#[test]
fn mul_properties() {
    // x * y is valid.
    // x * &y is valid.
    // &x * y is valid.
    // &x * &y is valid.
    // x * y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x *= y, x *= &y, x * y, x * &y, &x * y, and &x * &y give the same result.
    // x * y == y * x
    //TODO x * y / y == x and x * y / x == y
    // if x != 0 and y != 0, x * y >= x and x * y >= y
    #[allow(cyclomatic_complexity)]
    let two_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let raw_gmp_product = gmp_x.clone() * gmp_y.clone();
        assert!(raw_gmp_product.is_valid());
        let gmp_product = gmp_natural_to_native(&raw_gmp_product);
        let num_product = num_biguint_to_native_natural(
            &(native_natural_to_num_biguint(&x) * native_natural_to_num_biguint(&y)),
        );
        let rugint_product = rugint_integer_to_native_natural(
            &(native_natural_to_rugint_integer(&x) *
                  native_natural_to_rugint_integer(&y)),
        );

        let product_val_val = gmp_x.clone() * gmp_y.clone();
        let product_val_ref = gmp_x.clone() * &gmp_y;
        let product_ref_val = &gmp_x * gmp_y.clone();
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert_eq!(product_val_val, raw_gmp_product);
        assert_eq!(product_val_ref, raw_gmp_product);
        assert_eq!(product_ref_val, raw_gmp_product);

        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x._basecase_mul_assign_with_mem_opt(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product, "x: {}, y: {}", x, y);

        let mut mut_x = gmp_x.clone();
        mut_x *= gmp_y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, raw_gmp_product);
        let mut mut_x = gmp_x.clone();
        mut_x *= &gmp_y;
        assert_eq!(mut_x, raw_gmp_product);
        assert!(mut_x.is_valid());

        let mut mut_x = native_natural_to_rugint_integer(&x);
        mut_x *= native_natural_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_native_natural(&mut_x), product);

        let reverse_product = &y * &x;
        //TODO let inv_1 = (&product / &x).unwrap();
        //TODO let inv_2 = (&product / &y).unwrap();
        assert_eq!(gmp_product, product);
        assert_eq!(num_product, product);
        assert_eq!(rugint_product, product);
        assert_eq!(reverse_product, product);
        //TODO assert_eq!(inv_1, y);
        //TODO assert_eq!(inv_2, x);

        if x != 0 && y != 0 {
            assert!(product >= x);
            assert!(product >= y);
        }
    };

    // x * (y: u32) == x * from(y)
    // (y: u32) * x == x * from(y)
    let natural_and_u32 = |gmp_x: gmp::Natural, y: u32| {
        let x = gmp_natural_to_native(&gmp_x);
        let primitive_product_1 = &x * y;
        let primitive_product_2 = y * &x;
        let product = x * native::Natural::from(y);
        assert_eq!(primitive_product_1, product);
        assert_eq!(primitive_product_2, product);
    };

    // x * 0 == 0
    // 0 * x == 0
    // x * 1 == x
    // 1 * x == x
    //TODO x * x == x ^ 2
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let x_old = x.clone();
        assert_eq!(&x * native::Natural::from(0u32), 0);
        assert_eq!(native::Natural::from(0u32) * 0, 0);
        let id_1 = &x * native::Natural::from(1u32);
        let id_2 = native::Natural::from(1u32) * &x;
        //TODO let double = &x * &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        //TODO assert_eq!(double, x_old.pow(2));
    };

    // (x * y) * z == x * (y * z)
    // x * (y + z) == x * y + x * z
    // (x + y) * z == x * z + y * z
    let three_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural, gmp_z: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let z = gmp_natural_to_native(&gmp_z);
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, x * &z + y * z);
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 2048)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 2048)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_and_u32(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 2048).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 2048))
        .take(LARGE_LIMIT)
    {
        three_naturals(x, y, z);
    }
}
