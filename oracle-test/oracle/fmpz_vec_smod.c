/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `V.balanced_mod(m) = R`, `(&V).balanced_mod(m) = R`, and
    `v := V; v.balanced_mod_assign(m); v = R` lines, where `V` and `R` are integer vectors and `m`
    a nonzero integer, against _fmpz_vec_scalar_smod_fmpz. Any other nonempty line is an error, as
    is an input with no lines at all.

    FLINT documents that function for `p > 0` only, and for a negative modulus its result is not
    a balanced remainder at all: it reduces into `[0, |p|)` and then subtracts `p` from every
    element, because `floor(p/2)` is negative. Malachite's balanced remainder depends only on
    `|m|`, so the oracle passes `|m|` to FLINT, which keeps every line in FLINT's contract without
    skipping any.
*/

#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;

static int
check_balanced_mod_line(char * line, int line_number)
{
    char * receiver;
    char * modulus;
    char * expected_str;
    if (!split_vector_scalar_line(line, "balanced_mod", "balanced_mod_assign", &receiver,
                                  &modulus, &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in _fmpz_vec_scalar_smod_fmpz test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    slong len, expected_len;
    fmpz * v = fmpz_vec_set_str_malachite(&len, receiver);
    fmpz * expected = fmpz_vec_set_str_malachite(&expected_len, expected_str);
    fmpz_t m;
    fmpz_init(m);
    if (v == NULL || expected == NULL || fmpz_set_str(m, modulus, 10) != 0 || fmpz_is_zero(m))
    {
        flint_printf("error in _fmpz_vec_scalar_smod_fmpz test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        fmpz_abs(m, m);
        fmpz * r = _fmpz_vec_init(len);
        _fmpz_vec_scalar_smod_fmpz(r, v, len, m);
        if (expected_len != len || !_fmpz_vec_equal(r, expected, len))
        {
            flint_printf("error in _fmpz_vec_scalar_smod_fmpz test, line %d. FLINT: ",
                         line_number);
            fmpz_vec_print_malachite(r, len);
            flint_printf("\n");
            result = 1;
        }
        _fmpz_vec_clear(r, len);
    }
    if (v != NULL)
    {
        _fmpz_vec_clear(v, len);
    }
    if (expected != NULL)
    {
        _fmpz_vec_clear(expected, expected_len);
    }
    fmpz_clear(m);
    return result;
}

int
run__fmpz_vec_scalar_smod_fmpz(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_balanced_mod_line);
    return result != 0 ? result : require_some_lines("_fmpz_vec_scalar_smod_fmpz", checked);
}
