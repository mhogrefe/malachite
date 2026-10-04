/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `harmonic_number(n) = h` lines against fmpq_harmonic_ui.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

static int
check_harmonic_line(char * line, int line_number)
{
    fmpq_t expected, actual;
    fmpq_init(expected);
    fmpq_init(actual);
    int result = 0;
    const char * prefix = "harmonic_number(";
    char * needle = strstr(line, ") = ");
    if (strncmp(line, prefix, strlen(prefix)) == 0 && needle != NULL)
    {
        *needle = '\0';
        const char * n_str = line + strlen(prefix);
        const char * h_str = needle + strlen(") = ");
        ulong n = strtoul(n_str, NULL, 10);
        int ok = fmpq_set_str(actual, h_str, 10) == 0;
        if (ok)
        {
            fmpq_harmonic_ui(expected, n);
            if (!fmpq_equal(expected, actual))
            {
                flint_printf("line %d: harmonic_number(%wu) mismatch\n", line_number, n);
                result = 1;
            }
        }
        else
        {
            flint_printf("line %d: unreadable value\n", line_number);
            result = 1;
        }
    }
    else
    {
        flint_printf("line %d: unreadable line\n", line_number);
        result = 1;
    }
    fmpq_clear(expected);
    fmpq_clear(actual);
    return result;
}

int
run_fmpq_harmonic(const char * arg)
{
    return for_each_line(arg, check_harmonic_line);
}
