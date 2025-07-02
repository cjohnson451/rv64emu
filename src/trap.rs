#![allow(dead_code, unused_variables)]
use crate::cpu::*;

pub enum Exception{
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StoreAMOPageFault,
}

impl Exception {
    pub fn is_fatal(&self) -> bool{
        match self {
            Exception::InstructionAddressMisaligned
            | Exception::InstructionAccessFault
            | Exception::LoadAccessFault
            | Exception::StoreAMOAddressMisaligned
            | Exception::StoreAMOAccessFault => true,
            _ => false,
        }
    }
}

pub trait Trap {
    fn exception_num(&self) -> u64;

    fn handle_trap(&self, cpu: &mut Cpu){
        let old_pc = cpu.pc.wrapping_sub(4);
        let except_num = self.exception_num();
        let mode = cpu.curr_mode;
        if (mode <= Mode::Supervisor) && ((cpu.load_csr(MEDELEG).wrapping_shr(except_num as u32)) & 1 != 0)
        {
            cpu.curr_mode = Mode::Supervisor;
            cpu.store_csr(SEPC, old_pc & !1);
            cpu.store_csr(SCAUSE, except_num);
            cpu.pc = cpu.load_csr(STVEC) & !1;
            cpu.store_csr(STVAL, 0);
            let mut sstatus = cpu.load_csr(SSTATUS);
            if (sstatus >> 1) & 1 == 1 { sstatus |= (1 << 5); }
            else { sstatus &= !(1 << 5); }
            sstatus &= !(1 << 1);
            sstatus &= !(1 << 8); 
            if mode == Mode::Supervisor { sstatus |= 1 << 8; }
            cpu.store_csr(SSTATUS, sstatus);
        }
        else {
            cpu.store_csr(MEPC, old_pc & !1);
            cpu.store_csr(MCAUSE, except_num);
            cpu.pc = cpu.load_csr(MTVEC) & !1;
            cpu.store_csr(MTVAL, 0);
            let mut mstatus = cpu.load_csr(MSTATUS);
            if (mstatus >> 3) & 1 == 1 { mstatus |= 1 << 7; }
            else { mstatus &= !(1 << 7); }
            mstatus &= !(1 << 3);
            mstatus &= !(0b11 << 11); 
            mstatus |= (mode as u64) << 11;
            cpu.store_csr(MSTATUS, mstatus);
        }
    }
}

impl Trap for Exception {
    fn exception_num(&self) -> u64 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAMOAddressMisaligned => 6,
            Exception::StoreAMOAccessFault => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StoreAMOPageFault => 15,
        }
    }
}
