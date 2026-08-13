/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `dedekind_sum(h, k) = s` lines against fmpq_dedekind_sum.
*/

#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

static int
check_dedekind_line(char * line, int line_number)
{
    fmpz_t h, k;
    fmpq_t expected, actual;
    fmpz_init(h);
    fmpz_init(k);
    fmpq_init(expected);
    fmpq_init(actual);
    int result = 1;
    const char * prefix = "dedekind_sum(";
    char * comma = strstr(line, ", ");
    char * close = comma == NULL ? NULL : strstr(comma, ") = ");
    if (strncmp(line, prefix, strlen(prefix)) == 0 && comma != NULL && close != NULL)
    {
        *comma = '\0';
        *close = '\0';
        if (fmpz_set_str(h, line + strlen(prefix), 10) == 0
            && fmpz_set_str(k, comma + 2, 10) == 0
            && fmpq_set_str(actual, close + strlen(") = "), 10) == 0)
        {
            fmpq_dedekind_sum(expected, h, k);
            if (fmpq_equal(expected, actual))
            {
                result = 0;
            }
            else
            {
                flint_printf("line %d: dedekind_sum mismatch\n", line_number);
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
    fmpz_clear(h);
    fmpz_clear(k);
    fmpq_clear(expected);
    fmpq_clear(actual);
    return result;
}

int
run_fmpq_dedekind_sum(const char * arg)
{
    return for_each_line(arg, check_dedekind_line);
}
