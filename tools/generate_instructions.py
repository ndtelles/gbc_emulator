import itertools

output = ''

# 8-bit transfer instructions
LD_REG_CODES = {
    'B': '000',
    'C': '001',
    'D': '010',
    'E': '011',
    'H': '100',
    'L': '101',
    'A': '111',
}

for dest, src in itertools.product(LD_REG_CODES.keys(), repeat=2):
    instr_bin = ('01' + LD_REG_CODES[dest] + LD_REG_CODES[src])
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::LdRegToReg{{dest: Register::{dest}, src: Register::{src}}},\n'

for dest in LD_REG_CODES.keys():
    instr_bin = ('00' + LD_REG_CODES[dest] + '110')
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::LdImmediateDataToReg{{dest: Register::{dest}}},\n'

for dest in LD_REG_CODES.keys():
    instr_bin = ('01' + LD_REG_CODES[dest] + '110')
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::LdRegPairAddrToReg{{dest: Register::{dest}, src: (Register::H, Register::L)}},\n'

output += f'        0x0A => Operation::LdRegPairAddrToReg{{dest: Register::A, src: (Register::B, Register::C)}},\n'
output += f'        0x1A => Operation::LdRegPairAddrToReg{{dest: Register::A, src: (Register::D, Register::E)}},\n'

for src in LD_REG_CODES.keys():
    instr_bin = ('01110' + LD_REG_CODES[src])
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::LdRegToRegPairAddr{{dest: (Register::H, Register::L), src: Register::{src}}},\n'

output += f'        0x02 => Operation::LdRegToRegPairAddr{{dest: (Register::B, Register::C), src: Register::A}},\n'
output += f'        0x12 => Operation::LdRegToRegPairAddr{{dest: (Register::D, Register::E), src: Register::A}},\n'

output += f'        0x36 => Operation::LdImmediateDataToHLAddr,\n'

output += f'        0xF2 => Operation::LdRegCAddrToRegA,\n'
output += f'        0xE2 => Operation::LdRegAToRegCAddr,\n'

output += f'        0xF0 => Operation::LdImmediateAddrToRegA,\n'
output += f'        0xE0 => Operation::LdRegAToImmediateAddr,\n'

output += f'        0xFA => Operation::LdImmediate16BitAddrToRegA,\n'
output += f'        0xEA => Operation::LdRegAToImmediate16BitAddr,\n'

output += f'        0x2A => Operation::LdHLAddrToRegAAndIncrement,\n'
output += f'        0x3A => Operation::LdHLAddrToRegAAndDecrement,\n'

output += f'        0x22 => Operation::LdRegAToHLAddrAndIncrement,\n'
output += f'        0x32 => Operation::LdRegAToHLAddrAndDecrement,\n'

# 16-bit transfer instructions
LD_REG_PAIR_CODES = {
    ('B', 'C'): '00',
    ('D', 'E'): '01',
    ('H', 'L'): '10'
}

for (dest_high, dest_low) in LD_REG_PAIR_CODES.keys():
    instr_bin = ('00' + LD_REG_PAIR_CODES[(dest_high, dest_low)] + '0001')
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::LdImmediate16BitDataToRegPair{{dest: (Register::{dest_high}, Register::{dest_low})}},\n'

output += f'        0x31 => Operation::LdImmediate16BitDataToSP,\n'

output += f'        0xF9 => Operation::LdRegHLToSP,\n'

PUSH_REG_PAIR_CODES = {
    ('B', 'C'): '00',
    ('D', 'E'): '01',
    ('H', 'L'): '10',
    ('A', 'F'): '11'
}

for (src_high, src_low) in PUSH_REG_PAIR_CODES.keys():
    instr_bin = ('11' + PUSH_REG_PAIR_CODES[(src_high, src_low)] + '0101')
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::PushRegPairToStack{{src: (Register::{src_high}, Register::{src_low})}},\n'

for (dest_high, dest_low) in PUSH_REG_PAIR_CODES.keys():
    instr_bin = ('11' + PUSH_REG_PAIR_CODES[(dest_high, dest_low)] + '0001')
    instr_hex = '{:02X}'.format(int(instr_bin, 2))
    output += f'        0x{instr_hex} => Operation::PopStackToRegPair{{dest: (Register::{dest_high}, Register::{dest_low})}},\n'

print(output)
