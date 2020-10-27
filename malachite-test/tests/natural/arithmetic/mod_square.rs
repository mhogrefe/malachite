use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModNeg, ModSquare, ModSquareAssign, Square,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;
use malachite_test::inputs::natural::{
    pairs_of_naturals_var_2, positive_naturals, triples_of_naturals_var_4,
};

#[test]
fn mod_square_properties() {
    test_properties(pairs_of_naturals_var_2, |&(ref x, ref m)| {
        assert!(x.mod_is_reduced(m));
        let square_val_val = x.clone().mod_square(m.clone());
        let square_ref_val = x.mod_square(m.clone());
        let square_val_ref = x.clone().mod_square(m);
        let square = x.mod_square(m);
        assert!(square_val_val.is_valid());
        assert!(square_ref_val.is_valid());
        assert!(square_val_ref.is_valid());
        assert!(square.is_valid());
        assert!(square.mod_is_reduced(m));
        assert_eq!(square_val_val, square);
        assert_eq!(square_ref_val, square);
        assert_eq!(square_val_ref, square);

        let mut mut_x = x.clone();
        mut_x.mod_square_assign(m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);
        let mut mut_x = x.clone();
        mut_x.mod_square_assign(m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, square);

        assert_eq!(x.mod_mul(x, m), square);
        assert_eq!(x.square() % m, square);
        assert_eq!(x.mod_neg(m).mod_square(m), square);
    });

    test_properties(positive_naturals, |m| {
        assert_eq!(Natural::ZERO.mod_square(m), 0);
        if *m != 1 {
            assert_eq!(Natural::ONE.mod_square(m), 1);
        }
    });

    test_properties(triples_of_naturals_var_4, |&(ref x, ref y, ref m)| {
        assert_eq!(
            x.mod_mul(y, m).mod_square(m),
            x.mod_square(m).mod_mul(y.mod_square(m), m)
        );
    });

    test_properties(pairs_of_unsigneds_var_5::<Limb>, |&(x, m)| {
        assert_eq!(
            x.mod_square(m),
            Natural::from(x).mod_square(Natural::from(m))
        );
    });
}
