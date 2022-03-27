use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::generators::{rational_gen, rational_pair_gen};
use std::str::FromStr;

#[test]
fn test_square() {
    let test = |x, out| {
        let u = Rational::from_str(x).unwrap();

        assert_eq!(u.clone().square().to_string(), out);
        assert_eq!((&u).square().to_string(), out);

        let mut x = u;
        x.square_assign();
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("10", "100");
    test("123", "15129");
    test("1/2", "1/4");
    test("22/7", "484/49");

    test("-1", "1");
    test("-10", "100");
    test("-123", "15129");
    test("-1/2", "1/4");
    test("-22/7", "484/49");
}

#[test]
fn square_properties() {
    rational_gen().test_properties(|x| {
        let square = (&x).square();
        assert!(square.is_valid());

        let mut mut_x = x.clone();
        mut_x.square_assign();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(&x * &x, square);
        assert_eq!((-&x).square(), square);
        assert!(square >= 0);
        if x != 0 {
            assert_eq!(square.cmp_abs(&x), x.partial_cmp_abs(&1).unwrap());
        }

        //TODO assert_eq!(square.checked_sqrt(), Some(x.abs()));
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let x_squared = (&x).square();
        let y_squared = (&y).square();
        let xy = &x * &y;
        assert_eq!((&x + &y).square(), &x_squared + &y_squared + (&xy << 1));
        assert_eq!((&x - &y).square(), &x_squared + &y_squared - (&xy << 1));
        assert_eq!(xy.square(), x_squared * y_squared);
    });

    integer_gen().test_properties(|x| {
        assert_eq!((&x).square(), Rational::from(x).square());
    });
}
