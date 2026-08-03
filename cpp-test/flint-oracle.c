/*
    Copyright © 2026 Mikhail Hogrefe

    Uses code adapted from the FLINT Library examples.

        Copyright © 2022 Fredrik Johansson

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    A differential-testing oracle: reads lines that a Malachite demo printed, recomputes each one
    with FLINT, and exits nonzero on the first disagreement. Each mode's input file is passed as
    the second argument. See README.md.
*/

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/ulong_extras.h>

int
main(int argc, char * argv[])
{
    if (argc < 2)
    {
        flint_printf("usage: flint-oracle <mode> [input-file]\n");
        return 2;
    }
    if (strcmp(argv[1], "n_primitive_root_prime") == 0)
    {
        FILE * fp;
        char * line = NULL;
        size_t len = 0;
        ssize_t read;
        fp = fopen(argv[2], "r");
        if (fp == NULL)
        {
            exit(EXIT_FAILURE);
        }
        ssize_t prefix_len = strlen("primitive_root_prime(");
        int i = 1;
        while ((read = getline(&line, &len, fp)) != -1)
        {
            char * suffix = line + prefix_len;
            ulong n = strtoul(suffix, &suffix, 10);
            suffix = strchr(suffix, '=');
            suffix += 1;
            ulong output = strtoul(suffix, &suffix, 10);
            ulong flint_output = n_primitive_root_prime(n);
            if (flint_output != output)
            {
                flint_printf(
                    "error in primitive_root_prime test, line %d, on input %lu. Malachite: %lu, "
                    "FLINT: %lu\n",
                    i, n, output, flint_output);
                return 1;
            }
            i += 1;
        }
        fclose(fp);
        if (line)
        {
            free(line);
        }
    }
    else if (strcmp(argv[1], "fmpz_sqrtmod") == 0)
    {
        FILE * fp;
        char * line = NULL;
        size_t len = 0;
        ssize_t read;
        fp = fopen(argv[2], "r");
        if (fp == NULL)
        {
            exit(EXIT_FAILURE);
        }
        fmpz_t a, p, b, expected;
        fmpz_init(a);
        fmpz_init(p);
        fmpz_init(b);
        fmpz_init(expected);
        int i = 1;
        while ((read = getline(&line, &len, fp)) != -1)
        {
            char * dot = strstr(line, ".mod_sqrt(");
            if (dot == NULL)
            {
                continue;
            }
            *dot = '\0';
            fmpz_set_str(a, line, 10);
            char * mstart = dot + strlen(".mod_sqrt(");
            char * mend = strchr(mstart, ')');
            *mend = '\0';
            fmpz_set_str(p, mstart, 10);
            char * rest = mend + 1; /* " = Some(r)" or " = None" */
            int has_expected = 0;
            char * some = strstr(rest, "Some(");
            if (some != NULL)
            {
                char * rstart = some + strlen("Some(");
                char * rend = strchr(rstart, ')');
                *rend = '\0';
                fmpz_set_str(expected, rstart, 10);
                has_expected = 1;
            }
            /* Documented divergence: even moduli in (50, 600) hit n_jacobi_unsigned with an even
               modulus, whose behavior is undefined; Malachite runs the exhaustive search
               instead. */
            if (fmpz_is_even(p) && fmpz_cmp_ui(p, 50) > 0 && fmpz_cmp_ui(p, 600) < 0
                && fmpz_cmp_ui(a, 1) > 0)
            {
                i += 1;
                continue;
            }
            int ok = fmpz_sqrtmod(b, a, p);
            if (ok != has_expected || (ok && !fmpz_equal(b, expected)))
            {
                flint_printf("error in fmpz_sqrtmod test, line %d. FLINT: ok=%d b=", i, ok);
                fmpz_print(b);
                flint_printf("\n");
                return 1;
            }
            i += 1;
        }
        fmpz_clear(a);
        fmpz_clear(p);
        fmpz_clear(b);
        fmpz_clear(expected);
        fclose(fp);
        if (line)
        {
            free(line);
        }
    }
    else if (strcmp(argv[1], "n_sqrtmod") == 0)
    {
        FILE * fp;
        char * line = NULL;
        size_t len = 0;
        ssize_t read;
        fp = fopen(argv[2], "r");
        if (fp == NULL)
        {
            exit(EXIT_FAILURE);
        }
        int i = 1;
        while ((read = getline(&line, &len, fp)) != -1)
        {
            char * suffix = line;
            ulong a = strtoul(suffix, &suffix, 10);
            suffix = strstr(suffix, ".mod_sqrt(");
            suffix += strlen(".mod_sqrt(");
            ulong p = strtoul(suffix, &suffix, 10);
            int has_expected = 0;
            ulong expected = 0;
            char * some = strstr(suffix, "Some(");
            if (some != NULL)
            {
                some += strlen("Some(");
                expected = strtoul(some, &some, 10);
                has_expected = 1;
            }
            /* Documented divergences: even moduli in (50, 600) hit n_jacobi_unsigned with an even
               modulus, whose behavior is undefined; and for p = 2^64 - 1 and 2^64 - 3 the
               exponents (p + 1) / 4 and (p + 3) / 8 wrap, while Malachite computes them exactly,
               as fmpz_sqrtmod does. */
            if ((p % 2 == 0 && p > 50 && p < 600 && a > 1) || p == UWORD_MAX
                || p == UWORD_MAX - 2)
            {
                i += 1;
                continue;
            }
            ulong flint_output = n_sqrtmod(a, p);
            /* n_sqrtmod returns 0 both for failure and for the root of 0 */
            int flint_ok = flint_output != 0 || a == 0;
            if (flint_ok != has_expected || (has_expected && flint_output != expected))
            {
                flint_printf(
                    "error in n_sqrtmod test, line %d, on input (%wu, %wu). FLINT: %wu\n",
                    i, a, p, flint_output);
                return 1;
            }
            i += 1;
        }
        fclose(fp);
        if (line)
        {
            free(line);
        }
    }
    else if (strcmp(argv[1], "sqrtmod_stress") == 0)
    {
        /* A memory-stress diagnostic, not a diff: repeated fmpz_sqrtmod calls with promotions.
           This caught heap corruption in a FLINT 3.3.0-dev snapshot's fmpz allocator. */
        fmpz_t a, p, b;
        fmpz_init(a);
        fmpz_init(p);
        fmpz_init(b);
        fmpz_set_str(p, "26959946667150639794667015087019630673557916260026308143510066298881", 10);
        for (int i = 0; i < atoi(argv[2]); i++)
        {
            fmpz_set_ui(a, 15241578750190521UL + i);
            fmpz_sqrtmod(b, a, p);
        }
        fmpz_clear(a);
        fmpz_clear(p);
        fmpz_clear(b);
        flint_printf("stress done\n");
    }
    else
    {
        flint_printf("unknown mode %s\n", argv[1]);
        return 2;
    }
    return 0;
}
