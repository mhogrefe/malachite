/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `x.to_height() = h` and `x.into_height() = h` lines against fmpq_height, and
    `x.height_significant_bits() = b` lines against fmpq_height_bits. The rational receiver
    prints as `p/q` or `p`, which fmpq_set_str reads directly.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpq.h>

#include "oracle.h"

static int
check_height_value_line(char * line, int line_number, const char * needle)
{
    char * start = strstr(line, needle);
    if (start == NULL)
    {
        return 0;
    }
    *start = '\0';
    char * expected_str = start + strlen(needle);
    int result = 0;
    fmpq_t x;
    fmpz_t height, expected;
    fmpq_init(x);
    fmpz_init(height);
    fmpz_init(expected);
    if (fmpq_set_str(x, line, 10) != 0)
    {
        flint_printf("error in fmpq_height test, line %d: unreadable rational\n", line_number);
        result = 1;
    }
    else
    {
        fmpz_set_str(expected, expected_str, 10);
        fmpq_height(height, x);
        if (!fmpz_equal(height, expected))
        {
            flint_printf("error in fmpq_height test, line %d. FLINT: ", line_number);
            fmpz_print(height);
            flint_printf("\n");
            result = 1;
        }
    }
    fmpq_clear(x);
    fmpz_clear(height);
    fmpz_clear(expected);
    return result;
}

static int
check_to_height_line(char * line, int line_number)
{
    return check_height_value_line(line, line_number, ".to_height() = ");
}

static int
check_into_height_line(char * line, int line_number)
{
    return check_height_value_line(line, line_number, ".into_height() = ");
}

static int
check_height_bits_line(char * line, int line_number)
{
    char * start = strstr(line, ".height_significant_bits() = ");
    if (start == NULL)
    {
        return 0;
    }
    *start = '\0';
    ulong expected = strtoull(start + strlen(".height_significant_bits() = "), NULL, 10);
    int result = 0;
    fmpq_t x;
    fmpq_init(x);
    if (fmpq_set_str(x, line, 10) != 0)
    {
        flint_printf("error in fmpq_height_bits test, line %d: unreadable rational\n",
                     line_number);
        result = 1;
    }
    else if (fmpq_height_bits(x) != expected)
    {
        flint_printf("error in fmpq_height_bits test, line %d. FLINT: %wu\n", line_number,
                     fmpq_height_bits(x));
        result = 1;
    }
    fmpq_clear(x);
    return result;
}

int
run_fmpq_height(const char * arg)
{
    int r = for_each_line(arg, check_to_height_line);
    if (r != 0)
    {
        return r;
    }
    return for_each_line(arg, check_into_height_line);
}

int
run_fmpq_height_bits(const char * arg)
{
    return for_each_line(arg, check_height_bits_line);
}
