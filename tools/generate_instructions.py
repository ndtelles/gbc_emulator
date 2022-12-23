for i in range(256):
    opcode_hex = '{:02X}'.format(i)
    # print(
    #     f'pub(super) fn instr_0x{opcode_hex}(&mut self, _mem: &mut VirtualMemory) {{')
    # print('    todo!();')
    # print('}\n')

    print(f'0x{opcode_hex} => CPU::instr_0x{opcode_hex},')
