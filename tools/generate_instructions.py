import itertools


def print_operation(opcode_bin: str, operation: str, cycles: int):
    opcode_hex = '{:02X}'.format(int(opcode_bin, 2))
    print(
        f'        0x{opcode_hex} => Operation {{ op: {operation}, cycles: {cycles}, }},')


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
    print_operation(
        instr_bin,
        f'OperationType::LD(OPDest::Register(Register::{dest}), OPSrc::Register(Register::{src}))',
        1
    )

for dest in LD_REG_CODES.keys():
    instr_bin = ('00' + LD_REG_CODES[dest] + '110')
    print_operation(
        instr_bin,
        f'OperationType::LD(OPDest::Register(Register::{dest}), OPSrc::PCImmediate)',
        2
    )

for dest in LD_REG_CODES.keys():
    instr_bin = ('01' + LD_REG_CODES[dest] + '110')
    print_operation(
        instr_bin,
        f'OperationType::LD(OPDest::Register(Register::{dest}), OPSrc::RegisterPairAsPointer(Register::H, Register::L))',
        2
    )
print_operation(
    '00001010',
    f'OperationType::LD(OPDest::Register(Register::A), OPSrc::RegisterPairAsPointer(Register::B, Register::C))',
    2
)
print_operation(
    '00011010',
    f'OperationType::LD(OPDest::Register(Register::A), OPSrc::RegisterPairAsPointer(Register::D, Register::E))',
    2
)

for src in LD_REG_CODES.keys():
    instr_bin = ('01110' + LD_REG_CODES[src])
    print_operation(
        instr_bin,
        f'OperationType::LD(OPDest::RegisterPairAsPointer(Register::H, Register::L), OPSrc::Register(Register::{src}))',
        2
    )
print_operation(
    '00000010',
    f'OperationType::LD(OPDest::RegisterPairAsPointer(Register::B, Register::C), OPSrc::Register(Register::A))',
    2
)
print_operation(
    '00010010',
    f'OperationType::LD(OPDest::RegisterPairAsPointer(Register::D, Register::E), OPSrc::Register(Register::A))',
    2
)

print_operation(
    '00110110',
    f'OperationType::LD(OPDest::RegisterPairAsPointer(Register::H, Register::L), OPSrc::PCImmediate)',
    3
)

print_operation(
    '11110010',
    f'OperationType::LD(OPDest::Register(Register::A), OPSrc::RegisterAsPointer(Register::C))',
    2
)
print_operation(
    '11100010',
    f'OperationType::LD(OPDest::RegisterAsPointer(Register::C), OPSrc::Register(Register::A))',
    2
)


print_operation(
    '11110000',
    f'OperationType::LD(OPDest::Register(Register::A), OPSrc::PCImmediateAsPointer)',
    3
)
print_operation(
    '11100000',
    f'OperationType::LD(OPDest::PCImmediateAsPointer, OPSrc::Register(Register::A))',
    3
)

print_operation(
    '11111010',
    f'OperationType::LD(OPDest::Register(Register::A), OPSrc::PCImmediateAsPointer16)',
    4
)
print_operation(
    '11101010',
    f'OperationType::LD(OPDest::PCImmediateAsPointer16, OPSrc::Register(Register::A))',
    4
)

print_operation(
    '00101010',
    f'OperationType::LDAndIncrementSrc(OPDest::Register(Register::A), OPSrc::RegisterPairAsPointer(Register::H, Register::L))',
    2
)
print_operation(
    '00111010',
    f'OperationType::LDAndDecrementSrc(OPDest::Register(Register::A), OPSrc::RegisterPairAsPointer(Register::H, Register::L))',
    2
)

print_operation(
    '00100010',
    f'OperationType::LDAndIncrementDest(OPDest::RegisterPairAsPointer(Register::H, Register::L), OPSrc::Register(Register::A))',
    2
)
print_operation(
    '00110010',
    f'OperationType::LDAndDecrementDest(OPDest::RegisterPairAsPointer(Register::H, Register::L), OPSrc::Register(Register::A))',
    2
)


# 16-bit transfer instructions
LD_REG_PAIR_CODES = {
    ('B', 'C'): '00',
    ('D', 'E'): '01',
    ('H', 'L'): '10'
}

for (dest_high, dest_low) in LD_REG_PAIR_CODES.keys():
    instr_bin = ('00' + LD_REG_PAIR_CODES[(dest_high, dest_low)] + '0001')
    print_operation(
        instr_bin,
        f'OperationType::LD16(OPDest16::RegisterPair(Register::{dest_high}, Register::{dest_low}), OPSrc16::PCImmediate16)',
        3
    )
print_operation(
    '00110001',
    f'OperationType::LD16(OPDest16::StackPointerRegister, OPSrc16::PCImmediate16)',
    3
)

print_operation(
    '11111001',
    f'OperationType::LD16(OPDest16::StackPointerRegister, OPSrc16::RegisterPair(Register::H, Register::L))',
    2
)

PUSH_REG_PAIR_CODES = {
    ('B', 'C'): '00',
    ('D', 'E'): '01',
    ('H', 'L'): '10',
    ('A', 'F'): '11'
}

for (src_high, src_low) in PUSH_REG_PAIR_CODES.keys():
    instr_bin = ('11' + PUSH_REG_PAIR_CODES[(src_high, src_low)] + '0101')
    print_operation(
        instr_bin,
        f'OperationType::PUSH(OPSrc16::RegisterPair(Register::{src_high}, Register::{src_low}))',
        4
    )

for (dest_high, dest_low) in PUSH_REG_PAIR_CODES.keys():
    instr_bin = ('11' + PUSH_REG_PAIR_CODES[(dest_high, dest_low)] + '0001')
    print_operation(
        instr_bin,
        f'OperationType::POP(OPDest16::RegisterPair(Register::{dest_high}, Register::{dest_low}))',
        3
    )
