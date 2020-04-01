use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_5, quadruples_of_unsigneds_var_1, triples_of_unsigneds_var_1,
};

fn mod_add_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, modulus)| {
        assert!(x.mod_is_reduced(&modulus));
        assert!(y.mod_is_reduced(&modulus));
        let sum = x.mod_add(y, modulus);
        assert!(sum.mod_is_reduced(&modulus));

        let mut x_alt = x;
        x_alt.mod_add_assign(y, modulus);
        assert_eq!(x_alt, sum);

        assert_eq!(sum.mod_sub(y, modulus), x);
        assert_eq!(sum.mod_sub(x, modulus), y);
        assert_eq!(y.mod_add(x, modulus), sum);
        assert_eq!(x.mod_sub(y.mod_neg(modulus), modulus), sum);
    });

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, modulus)| {
        assert_eq!(x.mod_add(T::ZERO, modulus), x);
        assert_eq!(T::ZERO.mod_add(x, modulus), x);
        assert_eq!(x.mod_add(x.mod_neg(modulus), modulus), T::ZERO);
    });

    test_properties(quadruples_of_unsigneds_var_1::<T>, |&(x, y, z, modulus)| {
        assert_eq!(
            x.mod_add(y, modulus).mod_add(z, modulus),
            x.mod_add(y.mod_add(z, modulus), modulus)
        );
    });
}

#[test]
fn mod_add_properties() {
    mod_add_properties_helper::<u8>();
    mod_add_properties_helper::<u16>();
    mod_add_properties_helper::<u32>();
    mod_add_properties_helper::<u64>();
    mod_add_properties_helper::<usize>();
}
