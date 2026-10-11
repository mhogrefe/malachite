/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `V.content_chained(x) = g` and `(&V).content_chained(&x) = g` lines, where `V` is an
    integer vector and `x` a natural number, against _fmpz_vec_content_chained. Any other nonempty
    line is an error, as is an input with no lines at all.
*/

#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;

static int
check_content_chained_line(char * line, int line_number)
{
    char * receiver;
    char * input;
    char * expected_str;
    if (!split_vector_scalar_line(line, "content_chained", NULL, &receiver, &input,
                                  &expected_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in _fmpz_vec_content_chained test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    if (input[0] == '&')
    {
        input++;
    }
    int result = 0;
    slong len;
    fmpz * v = fmpz_vec_set_str_malachite(&len, receiver);
    fmpz_t x, out, expected;
    fmpz_init(x);
    fmpz_init(out);
    fmpz_init(expected);
    if (v == NULL || fmpz_set_str(x, input, 10) != 0
        || fmpz_set_str(expected, expected_str, 10) != 0)
    {
        flint_printf("error in _fmpz_vec_content_chained test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else
    {
        _fmpz_vec_content_chained(out, v, len, x);
        if (!fmpz_equal(out, expected))
        {
            flint_printf("error in _fmpz_vec_content_chained test, line %d. FLINT: ",
                         line_number);
            fmpz_print(out);
            flint_printf("\n");
            result = 1;
        }
    }
    if (v != NULL)
    {
        _fmpz_vec_clear(v, len);
    }
    fmpz_clear(x);
    fmpz_clear(out);
    fmpz_clear(expected);
    return result;
}

int
run__fmpz_vec_content_chained(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_content_chained_line);
    return result != 0 ? result : require_some_lines("_fmpz_vec_content_chained", checked);
}
