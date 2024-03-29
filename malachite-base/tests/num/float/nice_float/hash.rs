use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::hash::hash;

fn hash_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}

#[test]
fn hash_properties() {
    apply_fn_to_primitive_floats!(hash_properties_helper);
}
