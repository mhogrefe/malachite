import os
import subprocess

def try_building():
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets'], cwd = 'malachite-base-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets'], cwd = 'malachite-nz-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets', '--features', '32_bit_limbs'], cwd = 'malachite-nz-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets'], cwd = 'malachite-base')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets', '--features', 'serde'], cwd = 'malachite-nz')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets', '--features', '32_bit_limbs', '--features', 'serde'], cwd = 'malachite-nz')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets'], cwd = 'malachite-test')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets', '--features', '32_bit_limbs'], cwd = 'malachite-test')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets'], cwd = '../rust-wheels')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all-targets', '--features', '32_bit_limbs'], cwd = '../rust-wheels')
    except subprocess.CalledProcessError:
        return False
    return True


def replace_and_build(filename, line_number, substring, replacement):
    with open(filename, 'r') as in_f:
        with open('temp.rs', 'w') as out_f:
            n = 1
            for line in in_f.readlines() :
                line = line.rstrip()
                if n == line_number:
                    out_f.write(line.replace(substring, replacement) + '\n')
                else:
                    out_f.write(line + '\n')
                n += 1
    assert subprocess.call('cp ' + filename + ' backup.rs', shell = True) == 0
    assert subprocess.call('mv temp.rs ' + filename, shell = True) == 0
    if try_building():
        assert subprocess.call('rm backup.rs', shell = True) == 0
        return True
    else:
        assert subprocess.call('mv backup.rs ' + filename, shell = True) == 0
        return False


def try_replacing(filename, substring, replacement, exceptions):
    start_line_number = 1
    while True:
        found_instance = False
        with open(filename) as f:
            n = 1
            for line in f.readlines():
                line = line.rstrip()
                if n >= start_line_number and substring in line and all([e not in line for e in exceptions]):
                    found_instance = True
                    break
                n += 1
        if found_instance:
            replace_and_build(filename, n, substring, replacement)
            start_line_number = n + 1
        else:
            return


assert try_building()
filename_list = []
for root, directories, filenames in os.walk('.'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
for root, directories, filenames in os.walk('../rust-wheels'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

for filename in filename_list:
    if '/malachite-test/' in filename or '/rust-wheels/' in filename:
        try_replacing(filename, 'pub fn', 'pub(crate) fn', ['pub fn main()'])
    try_replacing(filename, 'pub(crate) fn', 'fn', [])
    try_replacing(filename, 'pub struct', 'pub(crate) struct', [])
    try_replacing(filename, 'pub(crate) struct', 'struct', [])
