use malachite_base::num::arithmetic::traits::{Ceiling, Floor, FloorAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use malachite_q_test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q_test_util::generators::rational_gen;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_floor() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let floor = x.clone().floor();
        assert!(floor.is_valid());
        assert_eq!(floor.to_string(), out);

        let floor = (&x).floor();
        assert!(floor.is_valid());
        assert_eq!(floor.to_string(), out);

        assert_eq!(BigRational::from_str(s).unwrap().floor().to_string(), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().floor().to_string(), out);

        let mut x = x;
        x.floor_assign();
        assert!(floor.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("1", "1");
    test("-1", "-1");
    test("123", "123");
    test("22/7", "3");
    test("-22/7", "-4");
    test("936851431250/1397", "670616629");
}

#[test]
fn floor_properties() {
    rational_gen().test_properties(|x| {
        let floor = x.clone().floor();
        assert!(floor.is_valid());

        assert_eq!(
            bigrational_to_rational(&rational_to_bigrational(&x).floor()),
            floor
        );
        assert_eq!(
            rug_rational_to_rational(&rational_to_rug_rational(&x).floor()),
            floor
        );

        let floor_alt = (&x).floor();
        assert!(floor_alt.is_valid());
        assert_eq!(floor_alt, floor);

        let mut floor_alt = x.clone();
        floor_alt.floor_assign();
        assert!(floor_alt.is_valid());
        assert_eq!(floor_alt, floor);

        assert_eq!(Integer::rounding_from(&x, RoundingMode::Floor), floor);
        assert!(floor <= x);
        assert!(&floor + Integer::ONE > x);
        assert_eq!(floor, Rational::from(&floor).floor());
        assert_eq!(floor, -(-x).ceiling());
    });
}
