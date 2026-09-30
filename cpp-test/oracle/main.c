/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    A differential-testing oracle: reads lines that a Malachite demo printed, recomputes each one
    with FLINT, and exits nonzero on the first disagreement. The first argument selects the mode;
    the second is the mode's input (a file of demo output, or an iteration count for the stress
    modes). See README.md.
*/

#include <string.h>

#include "oracle.h"

typedef struct
{
    const char * name;
    int (* run)(const char * arg);
} oracle_mode;

static const oracle_mode modes[] = {
    {"n_primitive_root_prime", run_n_primitive_root_prime},
    {"fmpz_sqrtmod", run_fmpz_sqrtmod},
    {"n_sqrtmod", run_n_sqrtmod},
    {"sqrtmod_stress", run_sqrtmod_stress},
    {"fmpz_mod_divides", run_fmpz_mod_divides},
    {"fmpz_divides_mod_list", run_fmpz_divides_mod_list},
    {"fmpz_CRT", run_fmpz_CRT},
    {"fmpz_CRT_balanced", run_fmpz_CRT_balanced},
    {"fmpz_multi_CRT", run_fmpz_multi_CRT},
    {"fmpz_multi_CRT_balanced", run_fmpz_multi_CRT_balanced},
    {"fmpz_multi_mod_ui", run_fmpz_multi_mod_ui},
    {"fmpz_multi_CRT_ui", run_fmpz_multi_CRT_ui},
    {"fmpz_multi_CRT_ui_balanced", run_fmpz_multi_CRT_ui_balanced},
    {"fmpz_rfac", run_fmpz_rfac},
    {"fmpz_xgcd_partial", run_fmpz_xgcd_partial},
    {"fmpq_height", run_fmpq_height},
    {"fmpq_height_bits", run_fmpq_height_bits},
    {"fmpq_gcd", run_fmpq_gcd},
    {"fmpq_gcd_cofactors", run_fmpq_gcd_cofactors},
    {"fmpq_farey_neighbors", run_fmpq_farey_neighbors},
    {"arith_bell_number", run_arith_bell_number},
    {"arith_bell_number_vec", run_arith_bell_number_vec},
    {"arith_landau_function_vec", run_arith_landau_function_vec},
    {"fmpq_dedekind_sum", run_fmpq_dedekind_sum},
    {"fmpq_harmonic", run_fmpq_harmonic},
    {"fmpq_next_minimal", run_fmpq_next_minimal},
    {"fmpq_next_signed_minimal", run_fmpq_next_signed_minimal},
    {"fmpq_reconstruct", run_fmpq_reconstruct},
    {"fmpq_reconstruct_2", run_fmpq_reconstruct_2},
    {"fmpz_poly_scalar_smod_fmpz", run_fmpz_poly_scalar_smod_fmpz},
    {"fmpz_poly_scalar_mod_fmpz", run_fmpz_poly_scalar_mod_fmpz},
    {"fmpz_poly_get_nmod_poly", run_fmpz_poly_get_nmod_poly},
    {"fmpz_mod_poly_set_fmpz_poly", run_fmpz_mod_poly_set_fmpz_poly},
    {"fmpz_poly_evaluate_fmpz", run_fmpz_poly_evaluate_fmpz},
    {"fmpz_poly_evaluate_horner_fmpz", run_fmpz_poly_evaluate_horner_fmpz},
    {"fmpz_poly_evaluate_divconquer_fmpz", run_fmpz_poly_evaluate_divconquer_fmpz},
    {"fmpz_poly_evaluate_fmpq", run_fmpz_poly_evaluate_fmpq},
    {"fmpz_poly_evaluate_horner_fmpq", run_fmpz_poly_evaluate_horner_fmpq},
    {"fmpz_poly_evaluate_divconquer_fmpq", run_fmpz_poly_evaluate_divconquer_fmpq},
    {"fmpq_poly_evaluate_fmpq", run_fmpq_poly_evaluate_fmpq},
    {"fmpq_poly_evaluate_fmpz", run_fmpq_poly_evaluate_fmpz},
    {"fmpz_mod_poly_evaluate_fmpz", run_fmpz_mod_poly_evaluate_fmpz},
    {"fmpz_poly_evaluate_mod", run_fmpz_poly_evaluate_mod},
    {"fmpq_poly_add", run_fmpq_poly_add},
    {"fmpq_poly_sub", run_fmpq_poly_sub},
    {"fmpz_poly_add_series", run_fmpz_poly_add_series},
    {"fmpz_poly_sub_series", run_fmpz_poly_sub_series},
    {"fmpq_poly_add_series", run_fmpq_poly_add_series},
    {"fmpq_poly_sub_series", run_fmpq_poly_sub_series},
    {"fmpz_poly_bit_pack", run_fmpz_poly_bit_pack},
    {"fmpz_poly_bit_unpack", run_fmpz_poly_bit_unpack},
    {"fmpz_poly_bit_unpack_unsigned", run_fmpz_poly_bit_unpack_unsigned},
    {"fmpz_poly_mul", run_fmpz_poly_mul},
    {"fmpz_poly_mullow", run_fmpz_poly_mullow},
    {"fmpz_poly_sqr", run_fmpz_poly_sqr},
    {"fmpz_poly_sqrlow", run_fmpz_poly_sqrlow},
    {"_fmpz_poly_mul_classical", run__fmpz_poly_mul_classical},
    {"_fmpz_poly_mul", run__fmpz_poly_mul},
    {"_fmpz_poly_mullow_classical", run__fmpz_poly_mullow_classical},
    {"_fmpz_poly_mullow", run__fmpz_poly_mullow},
    {"_fmpz_poly_mulhigh_classical", run__fmpz_poly_mulhigh_classical},
    {"_fmpz_poly_mulmid_classical", run__fmpz_poly_mulmid_classical},
    {"_fmpz_poly_mulmid", run__fmpz_poly_mulmid},
    {"_fmpz_poly_sqr_classical", run__fmpz_poly_sqr_classical},
    {"_fmpz_poly_sqr", run__fmpz_poly_sqr},
    {"_fmpz_poly_sqrlow_classical", run__fmpz_poly_sqrlow_classical},
    {"_fmpz_poly_sqrlow", run__fmpz_poly_sqrlow},
    {"_fmpz_poly_mul_karatsuba", run__fmpz_poly_mul_karatsuba},
    {"_fmpz_poly_mullow_karatsuba", run__fmpz_poly_mullow_karatsuba},
    {"_fmpz_poly_mullow_karatsuba_n", run__fmpz_poly_mullow_karatsuba_n},
    {"_fmpz_poly_mulhigh_karatsuba_n", run__fmpz_poly_mulhigh_karatsuba_n},
    {"_fmpz_poly_mulhigh", run__fmpz_poly_mulhigh},
    {"_fmpz_poly_sqr_karatsuba", run__fmpz_poly_sqr_karatsuba},
    {"_fmpz_poly_sqrlow_karatsuba", run__fmpz_poly_sqrlow_karatsuba},
    {"_fmpz_poly_sqrlow_karatsuba_n", run__fmpz_poly_sqrlow_karatsuba_n},
    {"_fmpz_poly_mul_KS", run__fmpz_poly_mul_KS},
    {"_fmpz_poly_mullow_KS", run__fmpz_poly_mullow_KS},
    {"_fmpz_poly_mulmid_KS", run__fmpz_poly_mulmid_KS},
    {"_fmpz_poly_sqr_KS", run__fmpz_poly_sqr_KS},
    {"_fmpz_poly_sqrlow_KS", run__fmpz_poly_sqrlow_KS},
    {"_fmpz_poly_mul_mid_default_mpn_ctx", run__fmpz_poly_mul_mid_default_mpn_ctx},
    {"_fmpz_poly_mul_SS", run__fmpz_poly_mul_SS},
    {"_fmpz_poly_mullow_SS", run__fmpz_poly_mullow_SS},
    {"_fmpz_poly_mulmid_SS", run__fmpz_poly_mulmid_SS},
    {"_fmpz_poly_sqr_SS", run__fmpz_poly_sqr_SS},
    {"_fmpz_poly_sqrlow_SS", run__fmpz_poly_sqrlow_SS},
    {"mpn_addmod_2expp1_1", run_mpn_addmod_2expp1_1},
    {"flint_mpn_sumdiff_n", run_flint_mpn_sumdiff_n},
    {"mpn_normmod_2expp1", run_mpn_normmod_2expp1},
    {"mpn_negmod_2expp1", run_mpn_negmod_2expp1},
    {"mpn_mul_2expmod_2expp1", run_mpn_mul_2expmod_2expp1},
    {"mpn_div_2expmod_2expp1", run_mpn_div_2expmod_2expp1},
    {"fft_adjust", run_fft_adjust},
    {"fft_adjust_sqrt2", run_fft_adjust_sqrt2},
    {"butterfly_lshB", run_butterfly_lshB},
    {"butterfly_rshB", run_butterfly_rshB},
    {"fft_butterfly", run_fft_butterfly},
    {"ifft_butterfly", run_ifft_butterfly},
    {"fft_radix2", run_fft_radix2},
    {"ifft_radix2", run_ifft_radix2},
    {"fft_truncate1", run_fft_truncate1},
    {"fft_truncate", run_fft_truncate},
    {"ifft_truncate1", run_ifft_truncate1},
    {"ifft_truncate", run_ifft_truncate},
    {"fft_butterfly_sqrt2", run_fft_butterfly_sqrt2},
    {"ifft_butterfly_sqrt2", run_ifft_butterfly_sqrt2},
    {"fft_truncate_sqrt2", run_fft_truncate_sqrt2},
    {"ifft_truncate_sqrt2", run_ifft_truncate_sqrt2},
    {"fft_negacyclic", run_fft_negacyclic},
    {"ifft_negacyclic", run_ifft_negacyclic},
    {"fft_butterfly_twiddle", run_fft_butterfly_twiddle},
    {"ifft_butterfly_twiddle", run_ifft_butterfly_twiddle},
    {"fft_radix2_twiddle", run_fft_radix2_twiddle},
    {"ifft_radix2_twiddle", run_ifft_radix2_twiddle},
    {"fft_truncate1_twiddle", run_fft_truncate1_twiddle},
    {"ifft_truncate1_twiddle", run_ifft_truncate1_twiddle},
    {"fft_mfa_truncate_sqrt2_outer", run_fft_mfa_truncate_sqrt2_outer},
    {"ifft_mfa_truncate_sqrt2_outer", run_ifft_mfa_truncate_sqrt2_outer},
    {"fft_mfa_truncate_sqrt2_inner", run_fft_mfa_truncate_sqrt2_inner},
    {"flint_mpn_mulmod_2expp1_basecase", run_flint_mpn_mulmod_2expp1_basecase},
    {"fft_naive_convolution_1", run_fft_naive_convolution_1},
    {"_fft_mulmod_2expp1", run__fft_mulmod_2expp1},
    {"fft_mulmod_2expp1", run_fft_mulmod_2expp1},
    {"fft_adjust_limbs", run_fft_adjust_limbs},
    {"fft_split_limbs", run_fft_split_limbs},
    {"fft_split_bits", run_fft_split_bits},
    {"fft_combine_limbs", run_fft_combine_limbs},
    {"fft_combine_bits", run_fft_combine_bits},
    {"fft_convolution", run_fft_convolution},
    {"_fmpz_vec_get_fft", run__fmpz_vec_get_fft},
    {"_fmpz_vec_set_fft", run__fmpz_vec_set_fft},
};

int
main(int argc, char * argv[])
{
    if (argc < 3)
    {
        flint_printf("usage: flint-oracle <mode> <input-file-or-count>\n");
        return 2;
    }
    for (size_t i = 0; i < sizeof(modes) / sizeof(modes[0]); i++)
    {
        if (strcmp(argv[1], modes[i].name) == 0)
        {
            return modes[i].run(argv[2]);
        }
    }
    flint_printf("unknown mode %s\n", argv[1]);
    return 2;
}
