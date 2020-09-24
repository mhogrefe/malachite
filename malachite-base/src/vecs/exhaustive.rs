use iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use iterators::iterator_cache::IteratorCache;
use itertools::Itertools;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::SignificantBits;
use std::iter::{once, Once};
use std::mem::swap;

macro_rules! exhaustive_fixed_length_vecs {
    (
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_done: ident, $outputs: ident]),*
    ) => {
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<T: Clone, $($it: Iterator<Item=T>,)*> {
            done: bool,
            next: Vec<Option<T>>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
                $outputs: Vec<usize>,
            )*
            oi_map: Vec<usize>,
        }

        impl<T: Clone, $($it: Iterator<Item=T>,)*> Iterator for $exhaustive_struct<T, $($it,)*> {
            type Item = Vec<T>;

            fn next(&mut self) -> Option<Vec<T>> {
                if self.done {
                    None
                } else {
                    loop {
                        let mut some_are_valid = false;
                        let mut all_are_valid = true;
                        $(
                            let mut no_x = false;
                            for &output_index in &self.$outputs {
                                if let Some(x) = self.$xs.get(
                                    self.distributor.get_output(output_index)
                                ) {
                                    self.next[output_index] = Some(x.clone());
                                    some_are_valid = true;
                                } else {
                                    no_x = true;
                                    all_are_valid = false;
                                    break;
                                }
                            }
                            if no_x {
                                if !self.$xs_done {
                                    self.$xs_done = true;
                                    let xs_len = self.$xs.known_len().unwrap();
                                    if xs_len == 0 {
                                        self.done = true;
                                        return None;
                                    }
                                    self.distributor.set_max_bits(
                                        &self.$outputs,
                                        usize::wrapping_from(xs_len.significant_bits())
                                    );
                                    for x in &mut self.next {
                                        *x = None;
                                    }
                                    continue;
                                } else if some_are_valid {
                                    for x in &mut self.next {
                                        *x = None;
                                    }
                                    self.distributor.increment_counter();
                                    continue;
                                }
                            }
                        )*
                        if !some_are_valid {
                            self.done = true;
                            return None;
                        } else if all_are_valid {
                            break;
                        } else {
                            for x in &mut self.next {
                                *x = None;
                            }
                            self.distributor.increment_counter();
                        }
                    }
                    let mut out = vec![None; self.next.len()];
                    swap(&mut self.next, &mut out);
                    self.distributor.increment_counter();
                    Some(out.into_iter().map(Option::unwrap).collect())
                }
            }
        }

        pub fn $exhaustive_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
            output_types: &[(BitDistributorOutputType, usize)],
        ) -> $exhaustive_struct<T, $($it,)*> {
            $(
                let _max_input_index = $i;
            )*
            let oi_map: Vec<usize> = output_types.iter().map(|(_, i)| *i).collect();
            let oi_sorted_unique = oi_map.iter().cloned().unique().sorted().collect::<Vec<_>>();
            assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
            assert_eq!(*oi_sorted_unique.last().unwrap(), _max_input_index);
            $exhaustive_struct {
                done: false,
                next: vec![None; output_types.len()],
                distributor: BitDistributor::new(output_types.iter().map(|(ot, _)| *ot)
                    .collect::<Vec<_>>().as_slice()),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                    $outputs: output_types.iter().enumerate()
                        .filter_map(|(o, (_, i))| if *i == $i { Some(o) } else { None }).collect(),
                )*
                oi_map
            }
        }
    }
}

macro_rules! exhaustive_fixed_length_vecs_with_length_n {
    (
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_done: ident, $outputs: ident]),*
    ) => {
        exhaustive_fixed_length_vecs!(
            $exhaustive_struct,
            $exhaustive_fn,
            $([$i, $it, $xs, $xs_done, $outputs]),*
        );

        #[inline]
        pub fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_fn(
                $($xs,)*
                &[$((BitDistributorOutputType::normal(1), $i),)*]
            )
        }
    }
}

// No point in having `exhaustive_length_1_vecs`; `exhaustive_fixed_length_vecs_from_single` is more
// efficient
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs1,
    exhaustive_fixed_length_vecs_1_input,
    [0, I, xs, xs_done, outputs_0]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs2,
    exhaustive_fixed_length_vecs_2_inputs,
    exhaustive_length_2_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs3,
    exhaustive_fixed_length_vecs_3_inputs,
    exhaustive_length_3_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs4,
    exhaustive_fixed_length_vecs_4_inputs,
    exhaustive_length_4_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs5,
    exhaustive_fixed_length_vecs_5_inputs,
    exhaustive_length_5_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs6,
    exhaustive_fixed_length_vecs_6_inputs,
    exhaustive_length_6_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs7,
    exhaustive_fixed_length_vecs_7_inputs,
    exhaustive_length_7_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5],
    [6, O, ts, ts_done, outputs_6]
);
exhaustive_fixed_length_vecs_with_length_n!(
    ExhaustiveFixedLengthVecs8,
    exhaustive_fixed_length_vecs_8_inputs,
    exhaustive_length_8_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5],
    [6, O, ts, ts_done, outputs_6],
    [7, P, ss, ss_done, outputs_7]
);

#[derive(Clone, Debug)]
pub enum ExhaustiveFixedLengthVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(ExhaustiveFixedLengthVecs1<I::Item, I>),
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            ExhaustiveFixedLengthVecsFromSingle::Zero(ref mut xs) => xs.next(),
            ExhaustiveFixedLengthVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

pub fn exhaustive_fixed_length_vecs_from_single<I: Iterator>(
    len: usize,
    xs: I,
) -> ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => ExhaustiveFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => ExhaustiveFixedLengthVecsFromSingle::One(xs),
        len => ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(
            exhaustive_fixed_length_vecs_1_input(
                xs,
                &vec![(BitDistributorOutputType::normal(1), 0); len],
            ),
        ),
    }
}
