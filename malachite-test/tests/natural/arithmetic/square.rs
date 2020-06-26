use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::unsigneds_var_8;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

#[test]
fn square_properties() {
    test_properties(naturals, |x| {
        let square = x.square();
        assert!(square.is_valid());

        let mut mut_x = x.clone();
        mut_x.square_assign();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(x * x, square);
        assert!(square >= *x);
    });

    test_properties(pairs_of_naturals, |(x, y)| {
        assert_eq!((x * y).square(), x.square() * y.square());
    });

    test_properties_no_special(unsigneds_var_8::<Limb>, |&x| {
        assert_eq!(x.square(), Natural::from(x).square());
    });
}
