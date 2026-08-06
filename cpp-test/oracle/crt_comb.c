/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `comb_reduce([p, ...], x) = [r, ...]` lines against fmpz_multi_mod_ui, and
    `comb_combine([p, ...], [r, ...]) = x` and `comb_combine_balanced(...)` lines against
    fmpz_multi_CRT_ui at signs 0 and 1. The demos only print usable moduli lists, since
    fmpz_comb_init aborts on bad ones.
*/

#include <stdlib.h>
#include <string.h>

#include "oracle.h"

/* Parses `[a, b, c]` of ulongs at `*pp`, advancing past the closing bracket. The caller frees
   the array. */
static int
parse_ulong_list(char ** pp, ulong ** vec_out, slong * len_out)
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
    ulong * vec = FLINT_ARRAY_ALLOC(count, ulong);
    for (slong i = 0; i < count; i++)
    {
        char * end;
        vec[i] = strtoull(p, &end, 10);
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
check_comb_reduce_line(char * line, int line_number)
{
    char * start = strstr(line, "comb_reduce(");
    if (start == NULL)
    {
        return 0;
    }
    char * p = start + strlen("comb_reduce(");
    ulong * primes;
    slong len;
    if (!parse_ulong_list(&p, &primes, &len))
    {
        return 0;
    }
    p += 2;
    int result = 0;
    fmpz_t x;
    fmpz_init(x);
    char * close = strchr(p, ')');
    *close = '\0';
    fmpz_set_str(x, p, 10);
    p = close + 1;
    /* skip " = " */
    p += 3;
    ulong * expected;
    slong elen;
    if (!parse_ulong_list(&p, &expected, &elen) || elen != len)
    {
        flint_printf("error in fmpz_multi_mod_ui test, line %d: malformed line\n", line_number);
        flint_free(primes);
        fmpz_clear(x);
        return 1;
    }
    fmpz_comb_t comb;
    fmpz_comb_temp_t temp;
    fmpz_comb_init(comb, primes, len);
    fmpz_comb_temp_init(temp, comb);
    ulong * out = FLINT_ARRAY_ALLOC(len, ulong);
    fmpz_multi_mod_ui(out, x, comb, temp);
    for (slong i = 0; i < len; i++)
    {
        if (out[i] != expected[i])
        {
            flint_printf("error in fmpz_multi_mod_ui test, line %d. FLINT: out[%wd]=%wu\n",
                         line_number, i, out[i]);
            result = 1;
            break;
        }
    }
    fmpz_comb_temp_clear(temp);
    fmpz_comb_clear(comb);
    flint_free(primes);
    flint_free(expected);
    flint_free(out);
    fmpz_clear(x);
    return result;
}

static int
check_comb_combine_line(char * line, int line_number, const char * needle, int sign)
{
    char * start = strstr(line, needle);
    if (start == NULL)
    {
        return 0;
    }
    char * p = start + strlen(needle);
    ulong * primes;
    ulong * residues;
    slong len, rlen;
    if (!parse_ulong_list(&p, &primes, &len))
    {
        return 0;
    }
    p += 2;
    if (!parse_ulong_list(&p, &residues, &rlen) || rlen != len)
    {
        flint_printf("error in fmpz_multi_CRT_ui test, line %d: malformed line\n", line_number);
        flint_free(primes);
        return 1;
    }
    /* skip ") = " */
    p += 4;
    int result = 0;
    fmpz_t out, expected;
    fmpz_init(out);
    fmpz_init(expected);
    fmpz_set_str(expected, p, 10);
    fmpz_comb_t comb;
    fmpz_comb_temp_t temp;
    fmpz_comb_init(comb, primes, len);
    fmpz_comb_temp_init(temp, comb);
    fmpz_multi_CRT_ui(out, residues, comb, temp, sign);
    if (!fmpz_equal(out, expected))
    {
        flint_printf("error in fmpz_multi_CRT_ui test, line %d. FLINT: out=", line_number);
        fmpz_print(out);
        flint_printf("\n");
        result = 1;
    }
    fmpz_comb_temp_clear(temp);
    fmpz_comb_clear(comb);
    flint_free(primes);
    flint_free(residues);
    fmpz_clear(out);
    fmpz_clear(expected);
    return result;
}

static int
check_combine_line(char * line, int line_number)
{
    return check_comb_combine_line(line, line_number, "comb_combine(", 0);
}

static int
check_combine_balanced_line(char * line, int line_number)
{
    return check_comb_combine_line(line, line_number, "comb_combine_balanced(", 1);
}

int
run_fmpz_multi_mod_ui(const char * arg)
{
    return for_each_line(arg, check_comb_reduce_line);
}

int
run_fmpz_multi_CRT_ui(const char * arg)
{
    return for_each_line(arg, check_combine_line);
}

int
run_fmpz_multi_CRT_ui_balanced(const char * arg)
{
    return for_each_line(arg, check_combine_balanced_line);
}
