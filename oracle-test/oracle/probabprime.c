/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `n is a probable prime` and `n is not a probable prime` lines against
    fmpz_is_probabprime, and `n is a strong probable prime to base a` and `n is not a strong
    probable prime to base a` lines against fmpz_is_strong_probabprime. The fmpz_is_prime mode diffs
    `n is proven prime` and `n is proven composite` lines against fmpz_is_prime, and skips `n is not
    settled` lines, which Malachite's n - 1 and n + 1 tests leave for a full primality proof. The
    fmpz_divisor_in_residue_class_lenstra mode diffs `lenstra(n, r, s) = d` and `lenstra(n, r, s) =
    none` lines against fmpz_divisor_in_residue_class_lenstra, which must find the same divisor d,
    or none. All four modes are strict: a nonempty line of any other shape is an error, and so is an
    input in which no line was checked.
*/

#include <string.h>

#include <flint/fmpz.h>

#include "oracle.h"

static long probabprime_checked = 0;

static void
chomp(char * line)
{
    size_t len = strlen(line);
    while (len > 0 && (line[len - 1] == '\n' || line[len - 1] == '\r'))
    {
        line[--len] = '\0';
    }
}

/* If `line` ends with `suffix`, cuts the suffix off and returns 1. */
static int
cut_suffix(char * line, const char * suffix)
{
    size_t len = strlen(line), slen = strlen(suffix);
    if (len < slen || strcmp(line + len - slen, suffix) != 0)
    {
        return 0;
    }
    line[len - slen] = '\0';
    return 1;
}

/* Reads a nonnegative decimal integer that is the whole of `s`. */
static int
read_natural(fmpz_t x, const char * s)
{
    if (s[0] == '\0' || s[0] == '-')
    {
        return 0;
    }
    for (const char * p = s; *p; p++)
    {
        if (*p < '0' || *p > '9')
        {
            return 0;
        }
    }
    return fmpz_set_str(x, s, 10) == 0;
}

static int
check_fmpz_is_probabprime_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    int expected;
    if (cut_suffix(line, " is not a probable prime"))
    {
        expected = 0;
    }
    else if (cut_suffix(line, " is a probable prime"))
    {
        expected = 1;
    }
    else
    {
        flint_printf("error in fmpz_is_probabprime test, line %d: unrecognized line\n  %s\n",
            line_number, line);
        return 1;
    }
    fmpz_t n;
    fmpz_init(n);
    int result = 0;
    if (!read_natural(n, line))
    {
        flint_printf("error in fmpz_is_probabprime test, line %d: unreadable n\n  %s\n",
            line_number, line);
        result = 1;
    }
    else
    {
        int flint_output = fmpz_is_probabprime(n);
        probabprime_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in fmpz_is_probabprime test, line %d, on input %s. FLINT: %d, "
                "Malachite: %d\n", line_number, line, flint_output, expected);
            result = 1;
        }
    }
    fmpz_clear(n);
    return result;
}

int
run_fmpz_is_probabprime(const char * arg)
{
    probabprime_checked = 0;
    int result = for_each_line(arg, check_fmpz_is_probabprime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_is_probabprime", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_is_probabprime: %ld lines\n", probabprime_checked);
    return 0;
}

static int
check_fmpz_is_strong_probabprime_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    int expected;
    const char * negative = " is not a strong probable prime to base ";
    const char * positive = " is a strong probable prime to base ";
    char * base;
    char * mid;
    if ((mid = strstr(line, negative)) != NULL)
    {
        expected = 0;
        base = mid + strlen(negative);
    }
    else if ((mid = strstr(line, positive)) != NULL)
    {
        expected = 1;
        base = mid + strlen(positive);
    }
    else
    {
        flint_printf("error in fmpz_is_strong_probabprime test, line %d: unrecognized line\n"
            "  %s\n", line_number, line);
        return 1;
    }
    *mid = '\0';
    fmpz_t n, a;
    fmpz_init(n);
    fmpz_init(a);
    int result = 0;
    if (!read_natural(n, line) || !read_natural(a, base))
    {
        flint_printf("error in fmpz_is_strong_probabprime test, line %d: unreadable input\n",
            line_number);
        result = 1;
    }
    else
    {
        int flint_output = fmpz_is_strong_probabprime(n, a);
        probabprime_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in fmpz_is_strong_probabprime test, line %d, on input (%s, %s). "
                "FLINT: %d, Malachite: %d\n", line_number, line, base, flint_output, expected);
            result = 1;
        }
    }
    fmpz_clear(n);
    fmpz_clear(a);
    return result;
}

int
run_fmpz_is_strong_probabprime(const char * arg)
{
    probabprime_checked = 0;
    int result = for_each_line(arg, check_fmpz_is_strong_probabprime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_is_strong_probabprime", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_is_strong_probabprime: %ld lines\n", probabprime_checked);
    return 0;
}

static long prove_prime_unsettled = 0;

static int
check_fmpz_is_prime_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    int expected;
    if (cut_suffix(line, " is not settled"))
    {
        prove_prime_unsettled += 1;
        return 0;
    }
    else if (cut_suffix(line, " is proven composite"))
    {
        expected = 0;
    }
    else if (cut_suffix(line, " is proven prime"))
    {
        expected = 1;
    }
    else
    {
        flint_printf("error in fmpz_is_prime test, line %d: unrecognized line\n  %s\n",
            line_number, line);
        return 1;
    }
    fmpz_t n;
    fmpz_init(n);
    int result = 0;
    if (!read_natural(n, line))
    {
        flint_printf("error in fmpz_is_prime test, line %d: unreadable n\n  %s\n", line_number,
            line);
        result = 1;
    }
    else
    {
        int flint_output = fmpz_is_prime(n);
        probabprime_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in fmpz_is_prime test, line %d, on input %s. FLINT: %d, "
                "Malachite: %d\n", line_number, line, flint_output, expected);
            result = 1;
        }
    }
    fmpz_clear(n);
    return result;
}

int
run_fmpz_is_prime(const char * arg)
{
    probabprime_checked = 0;
    prove_prime_unsettled = 0;
    int result = for_each_line(arg, check_fmpz_is_prime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_is_prime", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_is_prime: %ld lines (%ld not settled, skipped)\n", probabprime_checked,
        prove_prime_unsettled);
    return 0;
}

static int
check_fmpz_divisor_in_residue_class_lenstra_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    char * args[4];
    char * p = line;
    const char * seps[4] = {"lenstra(", ", ", ", ", ") = "};
    for (int i = 0; i < 4; i++)
    {
        size_t len = strlen(seps[i]);
        char * q = (i == 0) ? (strncmp(p, seps[0], len) == 0 ? p : NULL) : strstr(p, seps[i]);
        if (q == NULL)
        {
            flint_printf("error in fmpz_divisor_in_residue_class_lenstra test, line %d: "
                "unrecognized line\n  %s\n", line_number, line);
            return 1;
        }
        *q = '\0';
        if (i > 0)
        {
            args[i - 1] = p;
        }
        p = q + len;
    }
    args[3] = p;
    fmpz_t n, r, s, d, fac;
    fmpz_init(n);
    fmpz_init(r);
    fmpz_init(s);
    fmpz_init(d);
    fmpz_init(fac);
    int result = 0;
    int none = strcmp(args[3], "none") == 0;
    if (!read_natural(n, args[0]) || !read_natural(r, args[1]) || !read_natural(s, args[2])
        || (!none && !read_natural(d, args[3])))
    {
        flint_printf("error in fmpz_divisor_in_residue_class_lenstra test, line %d: unreadable "
            "input\n", line_number);
        result = 1;
    }
    else
    {
        int found = fmpz_divisor_in_residue_class_lenstra(fac, n, r, s);
        probabprime_checked += 1;
        if (found == none || (found && !fmpz_equal(fac, d)))
        {
            flint_printf("error in fmpz_divisor_in_residue_class_lenstra test, line %d, on input "
                "(%s, %s, %s). FLINT: ", line_number, args[0], args[1], args[2]);
            if (found)
            {
                fmpz_print(fac);
            }
            else
            {
                flint_printf("none");
            }
            flint_printf(", Malachite: %s\n", args[3]);
            result = 1;
        }
    }
    fmpz_clear(n);
    fmpz_clear(r);
    fmpz_clear(s);
    fmpz_clear(d);
    fmpz_clear(fac);
    return result;
}

int
run_fmpz_divisor_in_residue_class_lenstra(const char * arg)
{
    probabprime_checked = 0;
    int result = for_each_line(arg, check_fmpz_divisor_in_residue_class_lenstra_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_divisor_in_residue_class_lenstra", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_divisor_in_residue_class_lenstra: %ld lines\n", probabprime_checked);
    return 0;
}
