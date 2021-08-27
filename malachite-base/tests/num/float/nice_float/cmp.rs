use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::common::test_cmp_helper;
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_pair_gen, primitive_float_triple_gen,
};
use std::cmp::Ordering;

const TEST_STRINGS: [&str; 7] = ["-Infinity", "-5.0e5", "-0.0", "NaN", "0.0", "0.123", "Infinity"];

#[test]
pub fn test_cmp() {
    test_cmp_helper::<NiceFloat<f32>>(&TEST_STRINGS);
    test_cmp_helper::<NiceFloat<f64>>(&TEST_STRINGS);
}

fn cmp_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let ord = x.cmp(&y);
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!(NiceFloat(-y.0).cmp(&NiceFloat(-x.0)), ord);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(x.cmp(&x), Ordering::Equal);
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let z = NiceFloat(z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}

#[test]
pub fn cmp_properties() {
    apply_fn_to_primitive_floats!(cmp_properties_helper);
}
