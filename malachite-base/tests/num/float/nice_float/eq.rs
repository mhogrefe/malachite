use malachite_base::num::float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::common::test_eq_helper;
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_pair_gen, primitive_float_triple_gen,
};

const TEST_STRINGS: [&str; 7] = [
    "-Infinity",
    "-5.0e5",
    "-0.0",
    "NaN",
    "0.0",
    "0.123",
    "Infinity",
];

#[test]
pub fn test_eq() {
    test_eq_helper::<NiceFloat<f32>>(&TEST_STRINGS);
    test_eq_helper::<NiceFloat<f64>>(&TEST_STRINGS);
}

#[allow(clippy::eq_op)]
fn eq_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        assert_eq!(x == y, y == x);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(x, x);
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let z = NiceFloat(z);
        if x == y && x == z {
            assert_eq!(x, z);
        }
    });
}

#[test]
pub fn eq_properties() {
    apply_fn_to_primitive_floats!(eq_properties_helper);
}
