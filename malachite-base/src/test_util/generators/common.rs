// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::marker::PhantomData;

pub const TINY_LIMIT: usize = 1000;
pub const SMALL_LIMIT: usize = 1000;
pub const LARGE_LIMIT: usize = 10000;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum GenMode {
    Exhaustive,
    Random,
    SpecialRandom,
}

impl GenMode {
    pub const fn name(self) -> &'static str {
        match self {
            GenMode::Exhaustive => "exhaustive",
            GenMode::Random => "random",
            GenMode::SpecialRandom => "special_random",
        }
    }
}

pub type It<T> = Box<dyn Iterator<Item = T>>;

#[derive(Clone, Debug)]
pub struct GenConfig(HashMap<String, u64>);

impl GenConfig {
    pub fn new() -> GenConfig {
        GenConfig(HashMap::new())
    }

    pub fn insert(&mut self, key: &str, value: u64) {
        self.0.insert(key.to_string(), value);
    }

    pub fn get_or(&self, key: &'static str, default: u64) -> u64 {
        *self.0.get(key).unwrap_or(&default)
    }
}

impl Default for GenConfig {
    fn default() -> GenConfig {
        GenConfig::new()
    }
}

pub struct Generator<T: 'static> {
    phantom: PhantomData<*const T>,
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
            phantom: PhantomData,
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
            phantom: PhantomData,
            exhaustive,
            random,
            special_random: None,
        }
    }

    fn test_properties_with_config_with_limit_and_optional_exhaustive_limit<F: FnMut(T)>(
        &self,
        config: &GenConfig,
        mut test: F,
        limit: usize,
        exhaustive_limit: bool,
    ) {
        if exhaustive_limit {
            for x in (self.exhaustive)().take(limit) {
                test(x);
            }
        } else {
            for x in (self.exhaustive)() {
                test(x);
            }
        }
        for x in (self.random)(config).take(limit) {
            test(x);
        }
        if let Some(special_random) = self.special_random {
            for x in special_random(config).take(limit) {
                test(x);
            }
        }
    }

    pub fn test_properties_with_limit<F: FnMut(T)>(&self, limit: usize, test: F) {
        self.test_properties_with_config_with_limit_and_optional_exhaustive_limit(
            &GenConfig::new(),
            test,
            limit,
            true,
        );
    }

    fn test_properties_with_config_optional_exhaustive_limit<F: FnMut(T)>(
        &self,
        config: &GenConfig,
        test: F,
        exhaustive_limit: bool,
    ) {
        self.test_properties_with_config_with_limit_and_optional_exhaustive_limit(
            config,
            test,
            LARGE_LIMIT,
            exhaustive_limit,
        );
    }

    pub fn test_properties_with_config<F: FnMut(T)>(&self, config: &GenConfig, test: F) {
        self.test_properties_with_config_optional_exhaustive_limit(config, test, true);
    }

    #[inline]
    pub fn test_properties<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config(&GenConfig::new(), test);
    }

    #[inline]
    pub fn test_properties_with_limit_and_no_exhaustive_limit<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config_optional_exhaustive_limit(&GenConfig::new(), test, false);
    }

    #[inline]
    pub fn test_properties_no_exhaustive_limit<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config_optional_exhaustive_limit(&GenConfig::new(), test, false);
    }

    pub fn get(&self, gm: GenMode, config: &GenConfig) -> It<T> {
        match gm {
            GenMode::Exhaustive => (self.exhaustive)(),
            GenMode::Random => (self.random)(config),
            GenMode::SpecialRandom => {
                (self
                    .special_random
                    .expect("special_random mode unsupported"))(config)
            }
        }
    }
}

pub fn permute_1_3_2<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C)>>,
) -> Box<dyn Iterator<Item = (A, C, B)>> {
    Box::new(it.map(|(a, b, c)| (a, c, b)))
}

pub fn permute_2_1<A: 'static, B: 'static>(
    it: Box<dyn Iterator<Item = (A, B)>>,
) -> Box<dyn Iterator<Item = (B, A)>> {
    Box::new(it.map(|(a, b)| (b, a)))
}

pub fn permute_3_1_4_2<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C, D)>>,
) -> Box<dyn Iterator<Item = (C, A, D, B)>> {
    Box::new(it.map(|(a, b, c, d)| (c, a, d, b)))
}

pub fn reshape_1_2_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = (A, (B, C))>>,
) -> Box<dyn Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|(a, (b, c))| (a, b, c)))
}

pub fn reshape_1_3_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = (A, (B, C, D))>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|(a, (b, c, d))| (a, b, c, d)))
}

pub fn reshape_2_1_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = ((A, B), C)>>,
) -> Box<dyn Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|((a, b), c)| (a, b, c)))
}

pub fn reshape_2_1_1_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = ((A, B), C, D)>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b), c, d)| (a, b, c, d)))
}

#[allow(clippy::type_complexity)]
pub fn reshape_2_2_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = ((A, B), (C, D))>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b), (c, d))| (a, b, c, d)))
}

pub fn reshape_3_1_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = ((A, B, C), D)>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b, c), d)| (a, b, c, d)))
}
