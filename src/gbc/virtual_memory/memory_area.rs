use std::borrow::Cow;

use enum_map::Enum;
use tracing::error;

#[derive(Enum)]
pub enum MemoryAreaName {
    PrgRomFixed,
    PrgRomBanked,
    Vram,
    ExternalRam,
    WorkRamFixed,
    WorkRamBanked,
    Oam,
    BGPalette,
    IORegisters,
    HighRam,
    IERegister,
}

pub enum MemoryPermission {
    None,
    ReadOnly,
    ReadAndWrite,
}

/**
 * MemoryArea reprsents the physical memory (including multiple banks)
 * between two virtual addresses
 */
pub struct MemoryArea {
    start_addr: u16,
    end_addr: u16,
    bank_size: usize,
    num_banks: usize,
    active_bank: usize,
    permission: MemoryPermission,
    data: Vec<u8>,
}

impl MemoryArea {
    pub fn new(
        start_addr: u16,
        end_addr: u16,
        num_banks: usize,
        permission: MemoryPermission,
    ) -> Self {
        debug_assert!(end_addr >= start_addr);
        let bank_size = (end_addr - start_addr + 1) as usize;
        Self {
            start_addr,
            end_addr,
            bank_size,
            num_banks,
            active_bank: 0,
            permission,
            data: vec![0x00; bank_size * num_banks],
        }
    }

    // Convert the u16 virtual address to an index in the data vec
    fn translate_virtual_address_to_data_index(&self, addr: u16) -> usize {
        (addr - self.start_addr) as usize + (self.bank_size * self.active_bank)
    }

    pub(super) fn get_end_addr(&self) -> u16 {
        self.end_addr
    }

    pub(super) fn get_active_bank(&self) -> usize {
        self.active_bank
    }

    pub(super) fn set_active_bank(&mut self, active_bank: usize) {
        self.active_bank = active_bank;
    }

    pub(super) fn set_permission(&mut self, permission: MemoryPermission) {
        self.permission = permission;
    }

    pub(super) fn read(&self, addr: u16) -> u8 {
        if let MemoryPermission::None = self.permission {
            return 0xFF;
        }
        let idx = self.translate_virtual_address_to_data_index(addr);
        if idx >= self.data.len() {
            error!(
                "Invalid Read Index. Active bank = {}. Bank Size = {}",
                self.active_bank, self.bank_size
            );
        }
        self.data[idx]
    }

    // Return cow which lets us return either a borrowed or owned value
    pub(super) fn read_bytes(&self, addr: u16, length_bytes: usize) -> Cow<[u8]> {
        if let MemoryPermission::None = self.permission {
            return Cow::from(vec![0xFF; length_bytes]);
        }
        let idx = self.translate_virtual_address_to_data_index(addr);
        let end_idx = idx + length_bytes;
        Cow::from(&self.data[idx..end_idx])
    }

    pub(super) fn write(&mut self, addr: u16, val: u8) {
        if let MemoryPermission::ReadOnly | MemoryPermission::None = self.permission {
            return;
        }
        let idx = self.translate_virtual_address_to_data_index(addr);
        if idx >= self.data.len() {
            error!("Invalid Write Index");
        }
        self.data[idx] = val;
    }

    pub(super) fn write_bytes(&mut self, addr: u16, vals: &[u8]) {
        if let MemoryPermission::ReadOnly | MemoryPermission::None = self.permission {
            return;
        }
        let idx = self.translate_virtual_address_to_data_index(addr);
        let end_idx = idx + vals.len();
        self.data.splice(idx..end_idx, vals.iter().cloned());
    }

    pub(super) fn fill_from_src(&mut self, src: &[u8]) {
        self.data.copy_from_slice(src);
    }

    pub(super) fn borrow_raw_data(&self) -> &[u8] {
        &self.data
    }
}
