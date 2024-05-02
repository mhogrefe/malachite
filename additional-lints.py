import os

MAX_LINE_LENGTH = 100

line_length_exceptions = set((
    # long Markdown table rows and/or links
    ('./malachite-base/src/lib.rs', 57),
    ('./malachite-base/src/num/arithmetic/mod.rs', 326),
    ('./malachite-base/src/num/arithmetic/mod.rs', 327),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1328),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1568),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1569),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1570),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1571),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 77),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 236),
    ('./malachite-base/src/num/arithmetic/round_to_multiple_of_power_of_2.rs', 110),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 148),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 150),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 434),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 610),
    ('./malachite-float/src/conversion/mod.rs', 224),
    ('./malachite-float/src/lib.rs', 16),
    ('./malachite-base/src/num/exhaustive/mod.rs', 1023),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 27),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 28),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 29),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 64),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 65),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 76),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 77),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 78),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 107),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 109),
    ('./malachite-nz/src/lib.rs', 28),
    ('./malachite-nz/src/lib.rs', 94),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 34),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 35),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 36),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 138),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 139),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 162),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 163),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 164),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 532),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 534),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 523),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 525),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 825),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 827),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 353),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 532),
    ('./malachite-nz/src/natural/conversion/mod.rs', 212),
    ('./malachite-q/src/arithmetic/mod.rs', 71),
    ('./malachite-q/src/arithmetic/mod.rs', 73),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 118),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 201),
    ('./malachite-q/src/lib.rs', 46),
))

def lint(filename):
    i = 1
    with open(filename) as f:
        for line in f.readlines():
            line = line.rstrip()
            is_exception = (filename, i) in line_length_exceptions
            if is_exception:
                if len(line) <= MAX_LINE_LENGTH:
                    raise ValueError(f'line not too long: {filename}: {i} {line}')
            elif len(line) > MAX_LINE_LENGTH:
                raise ValueError(f'line too long: {filename}: {i} {line}')
            i += 1
    return i - 1

filename_list = []
for root, directories, filenames in os.walk('.'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and '.history' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

line_count = 0
for filename in filename_list:
    line_count += lint(filename)
print(f'{line_count} lines checked')
