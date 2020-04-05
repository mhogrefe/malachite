use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds_var_5, triples_of_unsigneds_var_1};

fn mod_sub_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let diff = x.mod_sub(y, m);
        assert!(diff.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_sub_assign(y, m);
        assert_eq!(x_alt, diff);

        assert_eq!(diff.mod_add(y, m), x);
        assert_eq!(diff.mod_sub(x, m), y.mod_neg(m));
        assert_eq!(y.mod_sub(x, m), diff.mod_neg(m));
        assert_eq!(x.mod_add(y.mod_neg(m), m), diff);
    });

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, m)| {
        assert_eq!(x.mod_sub(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_sub(x, m), x.mod_neg(m));
        assert_eq!(x.mod_sub(x, m), T::ZERO);
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
