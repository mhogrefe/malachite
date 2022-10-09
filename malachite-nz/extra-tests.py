import subprocess

def substitute_constant(const_prefix, input_filename, output_filename, value):
    prefix = f'{const_prefix}: usize = '
    with open(input_filename, 'r') as in_f:
        with open(output_filename, 'w') as out_f:
            for line in in_f.readlines():
                line = line.rstrip()
                if line.startswith(prefix):
                    original_value = line[len(prefix):-1]
                    out_f.write(prefix + value + ';\n')
                else:
                    out_f.write(line + '\n')

def substitute_hgcd_reduce_threshold(input_filename, output_filename, value):
    substitute_constant('const HGCD_REDUCE_THRESHOLD', input_filename, output_filename, value)

path = 'src/natural/arithmetic/gcd/half_gcd.rs'
assert subprocess.call('cp ' + path + ' backup.rs', shell = True) == 0
substitute_hgcd_reduce_threshold('backup.rs', 'temp.rs', '200')
assert subprocess.call('mv temp.rs ' + path, shell = True) == 0
assert subprocess.call('cargo test --test lib --features 32_bit_limbs --features serde --features test_build -- test_limbs_gcd_reduced', shell = True) == 0
assert subprocess.call('cargo test --test lib --features serde --features test_build -- test_limbs_gcd_reduced', shell = True) == 0
assert subprocess.call('mv backup.rs ' + path, shell = True) == 0
