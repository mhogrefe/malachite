/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `V.to_element_lcm() = l` and `V.into_element_lcm() = l` lines, where `V` is an integer
    vector, against _fmpz_vec_lcm. Any other nonempty line is an error, as is an input with no
    lines at all.
*/

#include <string.h>

#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;

static int
check_lcm_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
    {
        line[n - 1] = '\0';
    }
    if (line[0] == '\0')
    {
        return 0;
    }
    char * call = strstr(line, ".to_element_lcm() = ");
    size_t call_len = strlen(".to_element_lcm() = ");
    if (call == NULL)
    {
        call = strstr(line, ".into_element_lcm() = ");
        call_len = strlen(".into_element_lcm() = ");
    }
    if (call == NULL)
    {
        flint_printf("error in _fmpz_vec_lcm test, line %d: unrecognized line\n", line_number);
        return 1;
    }
    checked++;
    *call = '\0';
    char * expected_str = call + call_len;
    int result = 0;
    slong len;
    fmpz * v = fmpz_vec_set_str_malachite(&len, line);
    fmpz_t out, expected;
    fmpz_init(out);
    fmpz_init(expected);
    if (v == NULL || fmpz_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in _fmpz_vec_lcm test, line %d: unreadable input\n", line_number);
        result = 1;
    }
    else
    {
        _fmpz_vec_lcm(out, v, len);
        if (!fmpz_equal(out, expected))
        {
            flint_printf("error in _fmpz_vec_lcm test, line %d. FLINT: ", line_number);
            fmpz_print(out);
            flint_printf("\n");
            result = 1;
        }
    }
    if (v != NULL)
    {
        _fmpz_vec_clear(v, len);
    }
    fmpz_clear(out);
    fmpz_clear(expected);
    return result;
}

int
run__fmpz_vec_lcm(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_lcm_line);
    return result != 0 ? result : require_some_lines("_fmpz_vec_lcm", checked);
}
