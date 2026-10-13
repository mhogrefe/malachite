/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `V.mod_entrywise_inverse(m) = R` and `(&V).mod_entrywise_inverse(m) = R` lines, where
    `V` is a vector of naturals reduced modulo `m`, `m` (optionally written `&m`) is positive, and
    `R` is `Some(W)` or `None`, against _nmod_vec_invert. Any other nonempty line is an error, as
    is an input with no lines at all.

    _nmod_vec_invert assumes, without checking, that every element is invertible, and its result
    is unspecified otherwise, while Malachite returns `None`. So the oracle first checks, with
    fmpz_gcd, that `R` is `None` exactly when some element is not coprime to `m`, and compares
    `W` with FLINT's inverses only when every element is invertible. Three more cases keep every
    line checked rather than skipped:
    - modulo 1, every element is 0, which is its own inverse, so `W` must equal `V` (FLINT's
      `nmod_inv` is not meant for the modulus 1);
    - a modulus of more than one word has no vector function in FLINT (`fmpz_mod_vec.h` has no
      inversion), so those lines are checked element by element against fmpz_invmod;
    - every other line goes to _nmod_vec_invert.
    The counts of lines checked each way are printed, so a demo that never reaches a path shows.
*/

#include <string.h>
#include <flint/fmpz_vec.h>
#include <flint/nmod_vec.h>

#include "oracle.h"

static long checked;
static long none_lines;
static long modulus_one_lines;
static long word_lines;
static long multiword_lines;

static void
report_mismatch(int line_number, const fmpz * r, slong len)
{
    flint_printf("error in _nmod_vec_invert test, line %d. FLINT: ", line_number);
    fmpz_vec_print_malachite(r, len);
    flint_printf("\n");
}

/* Checks a `Some(W)` line whose elements are all invertible; returns 0 if W is right. */
static int
check_inverses(int line_number, const fmpz * v, const fmpz * w, slong len, const fmpz_t m)
{
    int result = 0;
    fmpz * r = _fmpz_vec_init(len);
    if (fmpz_is_one(m))
    {
        modulus_one_lines++;
        _fmpz_vec_set(r, v, len);
    }
    else if (fmpz_abs_fits_ui(m))
    {
        word_lines++;
        nmod_t mod;
        nmod_init(&mod, fmpz_get_ui(m));
        nn_ptr xs = _nmod_vec_init(len);
        nn_ptr ys = _nmod_vec_init(len);
        for (slong i = 0; i < len; i++)
        {
            xs[i] = fmpz_get_ui(v + i);
        }
        _nmod_vec_invert(ys, xs, len, mod);
        for (slong i = 0; i < len; i++)
        {
            fmpz_set_ui(r + i, ys[i]);
        }
        _nmod_vec_clear(xs);
        _nmod_vec_clear(ys);
    }
    else
    {
        multiword_lines++;
        for (slong i = 0; i < len; i++)
        {
            fmpz_invmod(r + i, v + i, m);
        }
    }
    if (!_fmpz_vec_equal(r, w, len))
    {
        report_mismatch(line_number, r, len);
        result = 1;
    }
    _fmpz_vec_clear(r, len);
    return result;
}

static int
check_line(char * line, int line_number)
{
    char * receiver;
    char * modulus;
    char * expected_str;
    if (!split_vector_scalar_line(line, "mod_entrywise_inverse", NULL, &receiver, &modulus,
                                  &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in _nmod_vec_invert test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    if (modulus[0] == '&')
    {
        modulus++;
    }
    int expect_some = 0;
    char * w_str = NULL;
    size_t expected_len_chars = strlen(expected_str);
    if (strcmp(expected_str, "None") == 0)
    {
        expect_some = 0;
    }
    else if (strncmp(expected_str, "Some(", 5) == 0 && expected_len_chars > 6
             && expected_str[expected_len_chars - 1] == ')')
    {
        expect_some = 1;
        expected_str[expected_len_chars - 1] = '\0';
        w_str = expected_str + 5;
    }
    else
    {
        flint_printf("error in _nmod_vec_invert test, line %d: unreadable result\n",
                     line_number);
        return 1;
    }
    int result = 0;
    slong len, w_len = 0;
    fmpz * v = fmpz_vec_set_str_malachite(&len, receiver);
    fmpz * w = expect_some ? fmpz_vec_set_str_malachite(&w_len, w_str) : NULL;
    fmpz_t m, g;
    fmpz_init(m);
    fmpz_init(g);
    if (v == NULL || (expect_some && w == NULL) || fmpz_set_str(m, modulus, 10) != 0
        || fmpz_sgn(m) <= 0)
    {
        flint_printf("error in _nmod_vec_invert test, line %d: unreadable input\n", line_number);
        result = 1;
        goto cleanup;
    }
    int all_invertible = 1;
    for (slong i = 0; i < len; i++)
    {
        if (fmpz_sgn(v + i) < 0 || fmpz_cmp(v + i, m) >= 0)
        {
            flint_printf("error in _nmod_vec_invert test, line %d: unreduced element\n",
                         line_number);
            result = 1;
            goto cleanup;
        }
        fmpz_gcd(g, v + i, m);
        if (!fmpz_is_one(g))
        {
            all_invertible = 0;
        }
    }
    if (expect_some != all_invertible)
    {
        flint_printf("error in _nmod_vec_invert test, line %d: Malachite says %s, but %s\n",
                     line_number, expect_some ? "Some" : "None",
                     all_invertible ? "every element is invertible"
                                    : "some element is not invertible");
        result = 1;
    }
    else if (!expect_some)
    {
        none_lines++;
    }
    else if (w_len != len)
    {
        flint_printf("error in _nmod_vec_invert test, line %d: wrong dimension\n", line_number);
        result = 1;
    }
    else
    {
        result = check_inverses(line_number, v, w, len, m);
    }
cleanup:
    if (v != NULL)
    {
        _fmpz_vec_clear(v, len);
    }
    if (w != NULL)
    {
        _fmpz_vec_clear(w, w_len);
    }
    fmpz_clear(m);
    fmpz_clear(g);
    return result;
}

int
run__nmod_vec_invert(const char * arg)
{
    checked = 0;
    none_lines = 0;
    modulus_one_lines = 0;
    word_lines = 0;
    multiword_lines = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("_nmod_vec_invert: %ld lines by _nmod_vec_invert, %ld by fmpz_invmod, %ld "
                     "modulo 1, %ld None\n",
                     word_lines, multiword_lines, modulus_one_lines, none_lines);
    }
    return result != 0 ? result : require_some_lines("_nmod_vec_invert", checked);
}
