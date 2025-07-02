use crate::dram::*;
use crate::dram::DRAM_BASE;
use crate::trap::*;

pub struct Bus{
    dram: Dram,
}

impl Bus{
    pub fn new(binary: Vec<u8>) -> Self{
        Self { dram: Dram::new(binary)}
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if addr >= DRAM_BASE{
            return self.dram.load(addr, size)
        }
        Err(Exception::LoadAccessFault)
    }
    
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if addr >= DRAM_BASE{
            return self.dram.store(addr, size, value)
        }
        Err(Exception::StoreAMOAccessFault)
    }
}