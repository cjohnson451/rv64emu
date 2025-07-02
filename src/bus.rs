use crate::dram::*;
use crate::trap::*;
use crate::plic::*;
use crate::clint::*;

pub const CLINT_BASE: u64 = 0x200_0000;
pub const CLINT_SIZE: u64 = 0x10000;
pub const PLIC_BASE: u64 = 0xc00_0000;
pub const PLIC_SIZE: u64 = 0x4000000;

pub trait Device {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception>;
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception>;
}

pub struct Bus{
    dram: Dram,
    plic: Plic,
    clint: Clint,
}

impl Bus{
    pub fn new(binary: Vec<u8>) -> Self{
        Self { 
            dram: Dram::new(binary),
            plic: Plic::new(),
            clint: Clint::new(),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.load(addr, size)
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.load(addr, size)
        }
        if addr >= DRAM_BASE{
            return self.dram.load(addr, size)
        }
        Err(Exception::LoadAccessFault)
    }
    
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.store(addr, size, value)
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.store(addr, size, value)
        }
        if addr >= DRAM_BASE{
            return self.dram.store(addr, size, value)
        }
        Err(Exception::StoreAMOAccessFault)
    }
}