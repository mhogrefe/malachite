use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_5, quadruples_of_unsigneds_var_1, triples_of_unsigneds_var_1,
};

fn mod_add_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let sum = x.mod_add(y, m);
        assert!(sum.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_add_assign(y, m);
        assert_eq!(x_alt, sum);

        assert_eq!(sum.mod_sub(y, m), x);
        assert_eq!(sum.mod_sub(x, m), y);
        assert_eq!(y.mod_add(x, m), sum);
        assert_eq!(x.mod_sub(y.mod_neg(m), m), sum);
    });

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, m)| {
        assert_eq!(x.mod_add(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_add(x, m), x);
        assert_eq!(x.mod_add(x.mod_neg(m), m), T::ZERO);
    });

    test_properties(quadruples_of_unsigneds_var_1::<T>, |&(x, y, z, m)| {
        assert_eq!(x.mod_add(y, m).mod_add(z, m), x.mod_add(y.mod_add(z, m), m));
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
