use crate::trap::*;
use crate::bus::*;

pub const CLINT_MTIMECMP: u64 = CLINT_BASE + 0x4000;
pub const CLINT_MTIME: u64 = CLINT_BASE + 0xbff8;

pub struct Clint {
    mtime: u64,
    mtimecmp: u64,
}

impl Device for Clint {
    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size == 64 {
            match addr {
                CLINT_MTIMECMP => {
                    return Ok(self.mtimecmp);
                }
                CLINT_MTIME => {
                    return Ok(self.mtime);
                }
                _ => {
                    return Ok(0);
                }
            }
        }   
        Err(Exception::LoadAccessFault)
    }
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size == 64 {
            match addr {
                CLINT_MTIMECMP => {
                    self.mtimecmp = value;
                }
                CLINT_MTIME => {
                    self.mtime = value
                }
                _ => {}
            }
            return Ok(());
        }   
        Err(Exception::StoreAMOAccessFault)
    }
}

impl Clint{
    pub fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
        }
    }
}