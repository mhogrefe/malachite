/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `multi_crt([m, ...], [v, ...]) = Some(x)|None` lines against fmpz_multi_CRT with
    sign = 0, and `multi_crt_balanced(...)` lines against fmpz_multi_CRT with sign = 1. FLINT's
    success flag maps to the Option: it reports failure exactly when the moduli list is unusable.
*/

#include <string.h>

#include <flint/fmpz_vec.h>

#include "oracle.h"

/* Parses `[a, b, c]` at `*pp`, advancing past the closing bracket. The caller clears the vector.
 */
static int
parse_fmpz_list(char ** pp, fmpz ** vec_out, slong * len_out)
{
    char * p = *pp;
    if (*p != '[')
    {
        return 0;
    }
    p++;
    slong count = 1;
    for (char * q = p; *q != '\0' && *q != ']'; q++)
    {
        if (*q == ',')
        {
            count++;
        }
    }
    fmpz * vec = _fmpz_vec_init(count);
    for (slong i = 0; i < count; i++)
    {
        char * end = strpbrk(p, ",]");
        char saved = *end;
        *end = '\0';
        fmpz_set_str(vec + i, p, 10);
        *end = saved;
        p = end;
        if (*p == ',')
        {
            p += 2;
        }
    }
    p++;
    *pp = p;
    *vec_out = vec;
    *len_out = count;
    return 1;
}

static int
check_multi_crt_line(char * line, int line_number, const char * needle, int sign)
{
    char * start = strstr(line, needle);
    if (start == NULL)
    {
        return 0;
    }
    char * p = start + strlen(needle);
    fmpz * moduli;
    fmpz * values;
    slong r, r2;
    if (!parse_fmpz_list(&p, &moduli, &r))
    {
        return 0;
    }
    p += 2;
    if (!parse_fmpz_list(&p, &values, &r2) || r2 != r)
    {
        flint_printf("error in fmpz_multi_CRT test, line %d: malformed lists\n", line_number);
        _fmpz_vec_clear(moduli, r);
        return 1;
    }
    p++;
    int result = 0;
    fmpz_t out, expected;
    fmpz_init(out);
    fmpz_init(expected);
    int has_expected = parse_option_fmpz(p, expected);
    int success = fmpz_multi_CRT(out, moduli, values, r, sign);
    if (success != has_expected || (success && !fmpz_equal(out, expected)))
    {
        flint_printf("error in fmpz_multi_CRT test, line %d. FLINT: success=%d out=",
                     line_number, success);
        fmpz_print(out);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(out);
    fmpz_clear(expected);
    _fmpz_vec_clear(moduli, r);
    _fmpz_vec_clear(values, r);
    return result;
}

static int
check_unsigned_line(char * line, int line_number)
{
    return check_multi_crt_line(line, line_number, "multi_crt(", 0);
}

static int
check_balanced_line(char * line, int line_number)
{
    return check_multi_crt_line(line, line_number, "multi_crt_balanced(", 1);
}

int
run_fmpz_multi_CRT(const char * arg)
{
    return for_each_line(arg, check_unsigned_line);
}

int
run_fmpz_multi_CRT_balanced(const char * arg)
{
    return for_each_line(arg, check_balanced_line);
}
