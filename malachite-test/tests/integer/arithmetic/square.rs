use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::signeds_var_2;
use malachite_test::inputs::integer::{integers, pairs_of_integers};
use malachite_test::inputs::natural::naturals;

#[test]
fn square_properties() {
    test_properties(integers, |x| {
        let square = x.square();
        assert!(square.is_valid());

        let mut mut_x = x.clone();
        mut_x.square_assign();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(x * x, square);
        assert_eq!((-x).square(), square);
        assert!(square >= 0);
        assert!(square >= *x);
    });

    test_properties(pairs_of_integers, |(x, y)| {
        assert_eq!((x * y).square(), x.square() * y.square());
    });

    test_properties(naturals, |x| {
        assert_eq!(x.square(), Integer::from(x).square());
    });

    test_properties_no_special(signeds_var_2::<SignedLimb>, |&x| {
        assert_eq!(x.square(), Integer::from(x).square());
    });
}
