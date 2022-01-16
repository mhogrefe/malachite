use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::generators::natural_pair_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::{
    rational_natural_natural_triple_gen, rational_natural_pair_gen,
    rational_rational_natural_triple_gen,
};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_natural() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u).map(Ordering::reverse), out);
    };
    test("0", "0", Some(Ordering::Equal));
    test("0", "5", Some(Ordering::Less));
    test("123", "123", Some(Ordering::Equal));
    test("123", "124", Some(Ordering::Less));
    test("123", "122", Some(Ordering::Greater));
    test("1000000000000", "123", Some(Ordering::Greater));
    test("123", "1000000000000", Some(Ordering::Less));
    test("1000000000000", "1000000000000", Some(Ordering::Equal));
    test("-1000000000000", "1000000000000", Some(Ordering::Less));
    test("-1000000000000", "0", Some(Ordering::Less));

    test("99/100", "1", Some(Ordering::Less));
    test("101/100", "1", Some(Ordering::Greater));
    test("22/7", "3", Some(Ordering::Greater));
    test("22/7", "4", Some(Ordering::Less));
    test("-99/100", "1", Some(Ordering::Less));
    test("-101/100", "1", Some(Ordering::Less));
    test("-22/7", "3", Some(Ordering::Less));
    test("-22/7", "4", Some(Ordering::Less));
}

#[test]
fn partial_cmp_natural_properties() {
    rational_natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp(&y);
        assert_eq!(x.cmp(&Rational::from(&y)), cmp.unwrap());
        assert_eq!(
            rational_to_rug_rational(&x).partial_cmp(&natural_to_rug_integer(&y)),
            cmp
        );
        assert_eq!(y.partial_cmp(&x), cmp.map(Ordering::reverse));
    });

    rational_rational_natural_triple_gen().test_properties(|(x, z, y)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    rational_natural_natural_triple_gen().test_properties(|(y, x, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Rational::from(&y)), Some(x.cmp(&y)));
    });
}
