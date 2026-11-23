/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `n is prime` and `n is not prime` lines, for a word-sized n, against n_is_prime. The
    mode is strict: a nonempty line of any other shape is an error, and so is an input in which
    no line was checked.
*/

#include <errno.h>
#include <stdlib.h>
#include <string.h>

#include <flint/ulong_extras.h>

#include "oracle.h"

static long n_is_prime_checked = 0;

static int
check_n_is_prime_line(char * line, int line_number)
{
    size_t len = strlen(line);
    while (len > 0 && (line[len - 1] == '\n' || line[len - 1] == '\r'))
    {
        line[--len] = '\0';
    }
    if (len == 0)
    {
        return 0;
    }
    int expected;
    char * suffix;
    if ((suffix = strstr(line, " is not prime")) != NULL && suffix[13] == '\0')
    {
        expected = 0;
    }
    else if ((suffix = strstr(line, " is prime")) != NULL && suffix[9] == '\0')
    {
        expected = 1;
    }
    else
    {
        flint_printf("error in n_is_prime test, line %d: unrecognized line\n  %s\n", line_number,
            line);
        return 1;
    }
    *suffix = '\0';
    char * end;
    errno = 0;
    ulong n = strtoul(line, &end, 10);
    if (end == line || *end != '\0' || errno != 0 || line[0] == '-')
    {
        flint_printf("error in n_is_prime test, line %d: unreadable n\n  %s\n", line_number, line);
        return 1;
    }
    int flint_output = n_is_prime(n);
    n_is_prime_checked += 1;
    if (flint_output != expected)
    {
        flint_printf(
            "error in n_is_prime test, line %d, on input %wu. FLINT: %d, Malachite: %d\n",
            line_number, n, flint_output, expected);
        return 1;
    }
    return 0;
}

int
run_n_is_prime(const char * arg)
{
    n_is_prime_checked = 0;
    int result = for_each_line(arg, check_n_is_prime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("n_is_prime", n_is_prime_checked))
    {
        return 1;
    }
    flint_printf("n_is_prime: %ld lines\n", n_is_prime_checked);
    return 0;
}
