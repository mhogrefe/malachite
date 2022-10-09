fn rle_encode<T: Clone + Eq>(xs: &[T]) -> Vec<(T, usize)> {
    let mut out = Vec::new();
    let mut previous: Option<T> = None;
    let mut count = 0;
    for x in xs.iter() {
        if let Some(p) = previous.as_ref() {
            if x == p {
                count += 1;
            } else {
                out.push((p.clone(), count));
                previous = Some(x.clone());
                count = 1;
            }
        } else {
            count = 1;
            previous = Some(x.clone());
        }
    }
    if let Some(p) = previous {
        out.push((p, count));
    }
    out
}

pub(crate) fn generate_rle_encoding() {
    // Example xs
    // let xs = &[1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2];
    let xs: &[u64] = &[
        1,
        0,
        0,
        0,
        0,
        0,
        0x2000000000000,
        0,
        0,
        0,
        0,
        14987979559889010688,
        u64::MAX,
        u64::MAX,
        18446743798831644671,
        u64::MAX,
        u64::MAX,
        383,
        0,
        0,
        0,
        0,
        18446744073709289472,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x7ffffff,
        0,
        0,
        0,
        0,
        18446743798831644672,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x1fffffffff,
        0,
        0,
        0,
        0,
        0xffff000000000000,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x7ffffffffffff,
        0,
        0,
        0,
        0,
        13835058055282163712,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX - 1,
        u64::MAX,
        18446744073709486079,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x1fffffff,
        0,
        0,
        0,
        0,
        18446742424442109952,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x7ffffffffffff,
        0,
        0,
        0,
        0,
        0xf000000000000000,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        18446742974197923839,
        18446744073709551591,
        u64::MAX,
        u64::MAX,
        18446744073701163007,
        0x7ffffffffffff,
        98304,
        0,
        0,
        0x800000000,
        0,
        18446744073608888320,
        u64::MAX,
        u64::MAX,
        18446708889337462783,
        u64::MAX,
        u64::MAX,
        18446744073708503039,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x1fffffffffffff,
        0x80000000,
        0,
        0,
        0,
        0,
        0,
        0,
        18428729675200069632,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        18446744073709550591,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0xffffffffffffff,
        0,
        0,
        0,
        0,
        0,
        0xfffffffffffffff0,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        16383,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x400000000,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x80000000000,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        64,
        0,
        0,
        0,
        0,
        18446744073709289472,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0xfffffff,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0xfffffffffff00000,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x7fffffff,
        0,
        0x400000000000000,
        0,
        0,
        0,
        8192,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        17870283321406128128,
        18446744073701163007,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        51539607679,
        0,
        0,
        0,
        0,
        18446638520593154048,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0xffffffffffffff,
        0,
        0,
        0,
        18446743936270598144,
        u64::MAX,
        511,
        0,
        0,
        0,
        0,
        0xfffffffffff00000,
        u64::MAX,
        18446603336221196287,
        18446744073709549567,
        u64::MAX,
        u64::MAX,
        18446744072635809791,
        u64::MAX,
        0x3fffff,
        0,
        0,
        0x20000000000,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        18446744073575342080,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        17293822569102704639,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x3ffffff,
        0,
        0,
        0,
        0,
        18446743661392691200,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        844424930131967,
        0,
        0,
        0,
        0,
        17870283321406128128,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        18446742974197923839,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0xfffffffffffff,
        0,
        0x2000000000000,
        0x4000,
        0,
        13835058055282163712,
        0x1ffffffff,
        0,
        18446744073675997184,
        u64::MAX,
        u64::MAX,
        18446726481523507199,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x3fffffff,
        0,
        0,
        0,
        0,
        18446744073575333888,
        u64::MAX,
        u64::MAX,
        18446603336221196287,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x1fffffff,
        0,
        0,
        0,
        0,
        18446741874686296064,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x7ffffffffffff,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        128,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x10000,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        18446744073709551488,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x3fffffff,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0xf000000000000000,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        383,
        0,
        0,
        0,
        0,
        18446744073708765184,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0x1fffffff,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        18446462598732316672,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        0xfffffffffffffff7,
        0x7ffffffffffffff,
        0,
        0,
        0,
        0x4000,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    println!("{:?}", rle_encode(xs));
}

fn limbs_mul_greater_to_out_toom_42_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    let n = if xs_len >= ys_len << 1 {
        xs_len.shr_round(2, RoundingMode::Ceiling)
    } else {
        ys_len.shr_round(1, RoundingMode::Ceiling)
    };
    let s = xs_len - 3 * n;
    let t = ys_len - n;
    assert!(n + 1 < xs_len);
    10 * n
        + 8
        + max!(
            limbs_mul_same_length_to_out_scratch_len(n),
            limbs_mul_same_length_to_out_scratch_len(n + 1),
            limbs_mul_to_out_scratch_len(s, t)
        )
}
