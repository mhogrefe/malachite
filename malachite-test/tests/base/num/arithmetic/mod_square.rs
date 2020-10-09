use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_5, positive_unsigneds, triples_of_unsigneds_var_1,
};

fn mod_square_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, m)| {
        assert!(x.mod_is_reduced(&m));
        let square = x.mod_square(m);
        assert!(square.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_square_assign(m);
        assert_eq!(x_alt, square);

        let data = T::precompute_mod_pow_data(&m);

        assert_eq!(x.mod_square_precomputed(m, &data), square);

        let mut x_alt = x;
        x_alt.mod_square_precomputed_assign(m, &data);
        assert_eq!(x_alt, square);

        assert_eq!(x.mod_mul(x, m), square);
        assert_eq!(x.mod_neg(m).mod_square(m), square);
    });

    test_properties(positive_unsigneds::<T>, |&m| {
        assert_eq!(T::ZERO.mod_square(m), T::ZERO);
        if m != T::ONE {
            assert_eq!(T::ONE.mod_square(m), T::ONE);
        }
    });

    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, m)| {
        assert_eq!(
            x.mod_mul(y, m).mod_square(m),
            x.mod_square(m).mod_mul(y.mod_square(m), m)
        );
    });
}

#[test]
fn mod_square_properties() {
    mod_square_properties_helper::<u8>();
    mod_square_properties_helper::<u16>();
    mod_square_properties_helper::<u32>();
    mod_square_properties_helper::<u64>();
    mod_square_properties_helper::<usize>();
}
