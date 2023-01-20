import os

MAX_LINE_LENGTH = 100

#def process_block(block):
#    words = []
#    (padding, block) = block
#    for (line_number, line) in block:
#        words.extend(line[4:].split(' '))
#    fixed_lines = []
#    current_line = ''
#    for i in range(0, padding):
#        current_line += ' '
#    current_line += '///'
#    for word in words:
#        if len(current_line) + len(word) + 1 > MAX_LINE_LENGTH:
#            fixed_lines.append(current_line)
#            current_line = ''
#            for i in range(0, padding):
#                current_line += ' '
#            current_line += '/// '
#            current_line += word
#        else:
#            current_line += ' '
#            current_line += word
#    fixed_lines.append(current_line)
#    print fixed_lines
#
#def lint(filename):
#    i = 1
#    blocks = []
#    current_block = []
#    in_doctest = False
#    previously_in_comment = False
#    with open(filename) as f:
#        for line in f.readlines():
#            line = line.rstrip()
#            line_length = len(line)
#            line = line.lstrip()
#            padding = len(line) - line_length
#            in_comment = line.startswith('///')
#            if in_comment:
#                if line == '/// ```':
#                    in_doctest = not in_doctest
#                elif not in_doctest and line == '///':
#                    if current_block:
#                        blocks.append((padding, current_block))
#                        current_block = []
#                elif not in_doctest:
#                    current_block.append((i, line))
#            if not in_comment and previously_in_comment and current_block:
#                blocks.append((padding, current_block))
#                current_block = []
#            previously_in_comment = in_comment
#            i += 1
#    for block in blocks:
#        process_block(block)
#    return i - 1

line_length_exceptions = set((
    # long Markdown table rows and/or links
    ('./malachite-base/src/lib.rs', 57),
    ('./malachite-base/src/num/arithmetic/mod.rs', 308),
    ('./malachite-base/src/num/arithmetic/mod.rs', 309),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1307),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1547),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1548),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1549),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1550),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 77),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 236),
    ('./malachite-base/src/num/arithmetic/round_to_multiple_of_power_of_2.rs', 102),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 156),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 158),
    ('./malachite-base/src/num/exhaustive/mod.rs', 1023),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 27),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 28),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 29),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 65),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 66),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 77),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 78),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 79),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 109),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 111),
    ('./malachite-nz/src/lib.rs', 28),
    ('./malachite-nz/src/lib.rs', 94),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 34),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 35),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 36),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 135),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 136),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 159),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 160),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 161),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 530),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 532),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 538),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 540),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 848),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 850),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 327),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 474),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 504),
    ('./malachite-nz/src/natural/conversion/mod.rs', 161),
    ('./malachite-q/src/arithmetic/mod.rs', 72),
    ('./malachite-q/src/arithmetic/mod.rs', 74),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 183),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 209),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 286),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 312),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 14),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 81),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 120),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 203),
    ('./malachite-q/src/exhaustive/mod.rs', 45),
    ('./malachite-q/src/exhaustive/mod.rs', 47),
    ('./malachite-q/src/lib.rs', 47),
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
