use crate::trap::*;
use crate::bus::*;

pub const PLIC_PENDING: u64 = PLIC_BASE + 0x1000;
pub const PLIC_SENABLE: u64 = PLIC_BASE + 0x2080;
pub const PLIC_SPRIORITY: u64 = PLIC_BASE + 0x201000;
pub const PLIC_SCLAIM: u64 = PLIC_BASE + 0x201004;

pub struct Plic {
    pending: u64,
    senable: u64,
    spriority: u64,
    sclaim: u64,
}

impl Device for Plic {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size == 32 {
            match addr {
                PLIC_PENDING => {
                    return Ok(self.pending);
                }
                PLIC_SENABLE => {
                    return Ok(self.senable);
                }
                PLIC_SPRIORITY => {
                    return Ok(self.spriority);
                }
                PLIC_SCLAIM => {
                    return Ok(self.sclaim);
                }
                _ => {
                    return Ok(0);
                }
            }
        }   
        Err(Exception::LoadAccessFault)
    }
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size == 32 {
            match addr {
                PLIC_PENDING => {
                    self.pending = value;
                }
                PLIC_SENABLE => {
                    self.senable = value
                }
                PLIC_SPRIORITY => {
                    self.spriority = value;
                }
                PLIC_SCLAIM => {
                    self.sclaim = value
                }
                _ => {}
            }
            return Ok(());
        }   
        Err(Exception::StoreAMOAccessFault)
    }
}

impl Plic{
    pub fn new() -> Self {
        Self {
            pending: 0,
            senable: 0,
            spriority: 0,
            sclaim: 0,
        }
    }
}