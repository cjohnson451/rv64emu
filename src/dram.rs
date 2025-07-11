use crate::trap::*;

pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;
pub const DRAM_BASE: u64 = 0x8000_0000;

pub struct Dram{
    pub dram: Vec<u8>,
}

impl Dram{
    pub fn new(code: Vec<u8>) -> Self{
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram[..code.len()].copy_from_slice(&code);
        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception>{
        match size{
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(Exception::LoadAccessFault)
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception>{
        match size{
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault)
        }
    }

    pub fn load8(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] as u64
    }
    
    pub fn load16(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
    }
    
    pub fn load32(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
    }
    
    pub fn load64(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56)
    }

    pub fn store8(&mut self, addr: u64, value: u64){
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = value as u8;
    }

    pub fn store16(&mut self, addr: u64, value: u64){
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    pub fn store32(&mut self, addr: u64, value: u64){
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    pub fn store64(&mut self, addr: u64, value: u64){
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8;
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
        self.dram[index + 4] = ((value >> 32) & 0xff) as u8;
        self.dram[index + 5] = ((value >> 40) & 0xff) as u8;
        self.dram[index + 6] = ((value >> 48) & 0xff) as u8;
        self.dram[index + 7] = ((value >> 56) & 0xff) as u8;
    }
}
