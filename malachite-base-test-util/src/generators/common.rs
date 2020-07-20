use std::collections::BTreeMap;
use std::marker::PhantomData;

pub const SMALL_LIMIT: usize = 1_000;
pub const LARGE_LIMIT: usize = 10_000;

pub type It<T> = Box<dyn Iterator<Item = T>>;

pub struct GenConfig(BTreeMap<&'static str, u64>);

impl GenConfig {
    pub fn get_or(&self, key: &'static str, default: u64) -> u64 {
        *self.0.get(key).unwrap_or(&default)
    }
}

pub struct Generator<T: 'static> {
    phantom_data: PhantomData<*const T>,
    exhaustive: &'static dyn Fn() -> It<T>,
    random: &'static dyn Fn(&GenConfig) -> It<T>,
    special_random: Option<&'static dyn Fn(&GenConfig) -> It<T>>,
}

impl<T> Generator<T> {
    pub fn new(
        exhaustive: &'static dyn Fn() -> It<T>,
        random: &'static dyn Fn(&GenConfig) -> It<T>,
        special_random: &'static dyn Fn(&GenConfig) -> It<T>,
    ) -> Generator<T> {
        Generator {
            phantom_data: PhantomData,
            exhaustive,
            random,
            special_random: Some(special_random),
        }
    }

    pub fn new_no_special(
        exhaustive: &'static dyn Fn() -> It<T>,
        random: &'static dyn Fn(&GenConfig) -> It<T>,
    ) -> Generator<T> {
        Generator {
            phantom_data: PhantomData,
            exhaustive,
            random,
            special_random: None,
        }
    }

    pub fn test_properties_with_config<F: FnMut(T)>(&self, config: &GenConfig, mut test: F) {
        for x in (self.exhaustive)().take(LARGE_LIMIT) {
            test(x);
        }
        for x in (self.random)(config).take(LARGE_LIMIT) {
            test(x);
        }
        if let Some(special_random) = self.special_random {
            for x in special_random(config).take(LARGE_LIMIT) {
                test(x);
            }
        }
    }

    #[inline]
    pub fn test_properties<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config(&GenConfig(BTreeMap::new()), test)
    }
}
