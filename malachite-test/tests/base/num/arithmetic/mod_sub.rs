use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds_var_5, triples_of_unsigneds_var_1};

fn mod_sub_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, modulus)| {
        assert!(x.mod_is_reduced(&modulus));
        assert!(y.mod_is_reduced(&modulus));
        let difference = x.mod_sub(y, modulus);
        assert!(difference.mod_is_reduced(&modulus));

        let mut x_alt = x;
        x_alt.mod_sub_assign(y, modulus);
        assert_eq!(x_alt, difference);

        assert_eq!(difference.mod_add(y, modulus), x);
        assert_eq!(difference.mod_sub(x, modulus), y.mod_neg(modulus));
        assert_eq!(y.mod_sub(x, modulus), difference.mod_neg(modulus));
        assert_eq!(x.mod_add(y.mod_neg(modulus), modulus), difference);
    });

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, modulus)| {
        assert_eq!(x.mod_sub(T::ZERO, modulus), x);
        assert_eq!(T::ZERO.mod_sub(x, modulus), x.mod_neg(modulus));
        assert_eq!(x.mod_sub(x, modulus), T::ZERO);
    });
}

#[test]
fn mod_sub_properties() {
    mod_sub_properties_helper::<u8>();
    mod_sub_properties_helper::<u16>();
    mod_sub_properties_helper::<u32>();
    mod_sub_properties_helper::<u64>();
    mod_sub_properties_helper::<usize>();
}
