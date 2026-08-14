/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `bell_number(n) = b` lines against arith_bell_number.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpz.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

static int
check_bell_line(char * line, int line_number)
{
    fmpz_t expected, actual;
    fmpz_init(expected);
    fmpz_init(actual);
    int result = 1;
    const char * prefix = "bell_number(";
    char * needle = strstr(line, ") = ");
    if (strncmp(line, prefix, strlen(prefix)) == 0 && needle != NULL)
    {
        *needle = '\0';
        const char * n_str = line + strlen(prefix);
        const char * b_str = needle + strlen(") = ");
        ulong n = strtoul(n_str, NULL, 10);
        if (fmpz_set_str(actual, b_str, 10) == 0)
        {
            arith_bell_number(expected, n);
            if (fmpz_equal(expected, actual))
            {
                result = 0;
            }
            else
            {
                flint_printf("line %d: bell_number(%wu) mismatch\n", line_number, n);
            }
        }
        else
        {
            flint_printf("line %d: unreadable value\n", line_number);
        }
    }
    else
    {
        flint_printf("line %d: unreadable line\n", line_number);
    }
    fmpz_clear(expected);
    fmpz_clear(actual);
    return result;
}

int
run_arith_bell_number(const char * arg)
{
    return for_each_line(arg, check_bell_line);
}

static int
check_bell_vec_line(char * line, int line_number)
{
    fmpz * vec;
    fmpz_t entry;
    slong n, k;
    int result = 1;
    const char * prefix = "bell_numbers_prefix(";
    char * needle = strstr(line, ") = [");
    if (strncmp(line, prefix, strlen(prefix)) == 0 && needle != NULL)
    {
        *needle = '\0';
        n = strtol(line + strlen(prefix), NULL, 10);
        char * cursor = needle + strlen(") = [");
        vec = _fmpz_vec_init(n);
        fmpz_init(entry);
        arith_bell_number_vec(vec, n);
        result = 0;
        for (k = 0; k < n; k++)
        {
            char * end = strpbrk(cursor, ",]");
            if (end == NULL)
            {
                flint_printf("line %d: truncated vector\n", line_number);
                result = 1;
                break;
            }
            *end = '\0';
            if (fmpz_set_str(entry, cursor, 10) != 0
                || !fmpz_equal(entry, vec + k))
            {
                flint_printf("line %d: bell_numbers_prefix entry %wd mismatch\n",
                             line_number, k);
                result = 1;
                break;
            }
            cursor = end + 1;
            while (*cursor == ' ')
                cursor++;
        }
        fmpz_clear(entry);
        _fmpz_vec_clear(vec, n);
    }
    else
    {
        flint_printf("line %d: unreadable line\n", line_number);
    }
    return result;
}

int
run_arith_bell_number_vec(const char * arg)
{
    return for_each_line(arg, check_bell_vec_line);
}
