/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs reductions of a polynomial modulo a positive integer `m`, each coefficient going into
    `[0, m)`, against three FLINT functions:

    - fmpz_poly_scalar_mod_fmpz, for `IntegerPolynomial::mod_op` by a `Natural` or any unsigned;
    - fmpz_poly_get_nmod_poly, which needs a word-sized modulus, for `IntegerPolynomial::mod_op`
      by an unsigned primitive of at most 64 bits;
    - fmpz_mod_poly_set_fmpz_poly, for `NaturalPolynomial`'s `%`, `%=`, `mod_op`, and `mod_assign`
      by a `Natural` or any unsigned.

    Lines have any of the shapes that split_polynomial_scalar_line recognizes, with the methods
    `mod_op` and `mod_assign` and the operator `%`. Any other nonempty line is an error, as is an
    input with no lines at all, so that a change to a demo's output format cannot make a run pass
    without checking anything.
*/

#include <string.h>

#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

/* The reduction under test: sets `r` to `p` with every coefficient reduced into [0, m). */
typedef void (* reducer)(fmpz_poly_t r, const fmpz_poly_t p, const fmpz_t m);

static void
reduce_scalar_mod(fmpz_poly_t r, const fmpz_poly_t p, const fmpz_t m)
{
    fmpz_poly_scalar_mod_fmpz(r, p, m);
}

static void
reduce_get_nmod_poly(fmpz_poly_t r, const fmpz_poly_t p, const fmpz_t m)
{
    nmod_poly_t q;
    nmod_poly_init(q, fmpz_get_ui(m));
    fmpz_poly_get_nmod_poly(q, p);
    fmpz_poly_set_nmod_poly_unsigned(r, q);
    nmod_poly_clear(q);
}

static void
reduce_fmpz_mod_poly(fmpz_poly_t r, const fmpz_poly_t p, const fmpz_t m)
{
    fmpz_mod_ctx_t ctx;
    fmpz_mod_poly_t q;
    fmpz_mod_ctx_init(ctx, m);
    fmpz_mod_poly_init(q, ctx);
    fmpz_mod_poly_set_fmpz_poly(q, p, ctx);
    fmpz_mod_poly_get_fmpz_poly(r, q, ctx);
    fmpz_mod_poly_clear(q, ctx);
    fmpz_mod_ctx_clear(ctx);
}

static const char * current_name;
static reducer current_reducer;
static int current_word_sized;
static long current_checked;

static int
check_reduction_line(char * line, int line_number)
{
    char * receiver;
    char * modulus;
    char * expected_str;
    if (!split_polynomial_scalar_line(line, "mod_op", "mod_assign", "%", &receiver, &modulus,
                                      &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name, line_number);
        return 1;
    }
    current_checked++;
    int result = 0;
    fmpz_poly_t p, expected, r;
    fmpz_t m;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    if (!fmpz_poly_set_str_malachite(p, receiver)
        || !fmpz_poly_set_str_malachite(expected, expected_str)
        || fmpz_set_str(m, modulus, 10) != 0 || fmpz_sgn(m) <= 0
        || (current_word_sized && !fmpz_abs_fits_ui(m)))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        current_reducer(r, p, m);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpz_poly_print_pretty(r, "x");
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(m);
    return result;
}

static int
run_reduction(const char * arg, const char * name, reducer f, int word_sized)
{
    current_name = name;
    current_reducer = f;
    current_word_sized = word_sized;
    current_checked = 0;
    int result = for_each_line(arg, check_reduction_line);
    return result != 0 ? result : require_some_lines(name, current_checked);
}

int
run_fmpz_poly_scalar_mod_fmpz(const char * arg)
{
    return run_reduction(arg, "fmpz_poly_scalar_mod_fmpz", reduce_scalar_mod, 0);
}

int
run_fmpz_poly_get_nmod_poly(const char * arg)
{
    return run_reduction(arg, "fmpz_poly_get_nmod_poly", reduce_get_nmod_poly, 1);
}

int
run_fmpz_mod_poly_set_fmpz_poly(const char * arg)
{
    return run_reduction(arg, "fmpz_mod_poly_set_fmpz_poly", reduce_fmpz_mod_poly, 0);
}
