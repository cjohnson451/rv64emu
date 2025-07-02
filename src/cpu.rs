#![allow(dead_code, unused_variables)]
use crate::bus::*;
use crate::dram::{DRAM_SIZE, DRAM_BASE};
use crate::trap::*;

//Machine-level CSRs 
pub const MSTATUS: usize = 0x300;
pub const MIE: usize = 0x304;
pub const MTVEC: usize = 0x305;
pub const MHARTID: usize = 0xf14;
pub const MEDELEG: usize = 0x302;
pub const MIDELEG: usize = 0x303;
pub const MCOUNTEREN: usize = 0x306;
pub const MSCRATCH: usize = 0x340;
pub const MEPC: usize = 0x341;
pub const MCAUSE: usize = 0x342;
pub const MTVAL: usize = 0x343;
pub const MIP: usize = 0x344;

//Supervisor-level CSRs 
pub const SSTATUS: usize = 0x100;
pub const SIE: usize = 0x104;
pub const STVEC: usize = 0x105;
pub const SSCRATCH: usize = 0x140;
pub const SEPC: usize = 0x141;
pub const SCAUSE: usize = 0x142;
pub const STVAL: usize = 0x143;
pub const SIP: usize = 0x144;
pub const SATP: usize = 0x180;

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode{
    User = 0x0,
    Supervisor = 0x1,
    Machine = 0x3
}

pub struct Cpu{
    pub registers: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
    pub csregs: [u64; 4096],
    pub curr_mode: Mode,
}

impl Cpu{
    pub fn new(binary: Vec<u8>) -> Self{
        let mut regs = [0; 32];
        regs[2] = DRAM_BASE + DRAM_SIZE;
        Self {
            registers: regs,
            pc: DRAM_BASE,
            bus: Bus::new(binary),
            csregs: [0; 4096],
            curr_mode: Mode::Machine,
        }
    }   

    pub fn fetch(&mut self) -> Result<u64, Exception> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(_e) => Err(Exception::InstructionAccessFault),
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception>{
        self.bus.load(addr, size)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception>{
        self.bus.store(addr, size, value)
    }

    pub fn load_csr(&self, addr: usize) -> u64{
        match addr{
            SIE => self.csregs[MIE] & self.csregs[MIDELEG],
            _ => self.csregs[addr],
        }
    }

    pub fn store_csr(&mut self, addr: usize, value: u64){
        match addr{
            SIE => {
                let mask = self.csregs[MIDELEG];
                self.csregs[MIE] = (self.csregs[MIE] & !mask) | (value & mask);
            }
            _ => self.csregs[addr] = value,
        }
    }

    pub fn execute(&mut self, instruction: u64) -> Result<(), Exception>{
        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let funct3 = ((instruction >> 12) & 0x07) as usize;
        let funct7 = (instruction >> 25) & 0x7f;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;

        match opcode{
            0x03 =>{
                let imm = ((instruction as i32 as i64) >> 20) as u64;
                let addr = self.registers[rs1].wrapping_add(imm);
                match funct3{
                    //lb
                    0x00 => {
                        let data = self.load(addr, 8)? as i8 as i64 as u64;
                        self.registers[rd] = data;
                    }
                    //lh
                    0x01 => {
                        let data  = self.load(addr, 16)? as i16 as i64 as u64;
                        self.registers[rd] = data;
                    }
                    //lw
                    0x02 => {
                        let data = self.load(addr, 32)? as i32 as i64 as u64;
                        self.registers[rd] = data;
                    }
                    //ld
                    0x03 => {
                        let data = self.load(addr, 64)?;
                        self.registers[rd] = data;
                    }
                    //lbu
                    0x04 => {
                        let data = self.load(addr, 8)?;
                        self.registers[rd] = data;
                    }
                    //lhu
                    0x05 => {
                        let data = self.load(addr, 16)?;
                        self.registers[rd] = data;
                    }
                    //lwu
                    0x06 => {
                        let data = self.load(addr, 32)?;
                        self.registers[rd] = data;
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    } 
                }
            }
            0x13 => {
                let imm = ((instruction as i32 as i64) >> 20) as u64;
                let shiftamt = (imm & 0x3f) as u32;
                match funct3 {
                    //addi
                    0x0 => {
                        self.registers[rd] = self.registers[rs1].wrapping_add(imm);
                    }
                    // slli
                    0x1 => {
                        self.registers[rd] = self.registers[rs1] << shiftamt;
                    }
                    // slti
                    0x2 => {
                        self.registers[rd] = if (self.registers[rs1] as i64) < (imm as i64) { 1 } else { 0 };
                    }
                    // sltiu
                    0x3 => {
                        self.registers[rd] = if self.registers[rs1] < imm { 1 } else { 0 };
                    }
                    // xori
                    0x4 => {
                        self.registers[rd] = self.registers[rs1] ^ imm;
                    }
                    0x5 => {
                        match funct7 >> 1 {
                            // srli
                            0x00 => self.registers[rd] = self.registers[rs1].wrapping_shr(shiftamt),
                            // srai
                            0x10 => {
                                self.registers[rd] = (self.registers[rs1] as i64).wrapping_shr(shiftamt) as u64
                            }
                            _ => {}
                        }
                    }
                    // ori
                    0x6 => self.registers[rd] = self.registers[rs1] | imm,
                    // andi
                    0x7 => self.registers[rd] = self.registers[rs1] & imm,
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //auipc
            0x17 => {
                let imm = (instruction & 0xfffff000) as i32 as i64 as u64;
                self.registers[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x1b => {
                let imm = ((instruction as i32 as i64) >> 20) as u64;
                let shiftamt = (imm & 0x1f) as u32;
                match funct3 {
                    //addiw
                    0x00 => {
                        self.registers[rd] = self.registers[rs1].wrapping_add(imm) as i32 as i64 as u64;
                    }
                    //slliw
                    0x01 => {
                        self.registers[rd] = self.registers[rs1].wrapping_shl(shiftamt) as i32 as i64 as u64;
                    }
                    0x05 => {
                        match funct7 {
                            //srliw
                            0x00 => {
                                self.registers[rd] = (self.registers[rs1] as u32).wrapping_shr(shiftamt) as i32 as i64 as u64;
                            }
                            //sraiw
                            0x20 => {
                                self.registers[rd] = (self.registers[rs1] as i32).wrapping_shr(shiftamt) as i64 as u64;
                            }
                            _ => {
                                eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                                return Err(Exception::IllegalInstruction)
                            }
                        }
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //sb, sh, sw, sd
            0x23 => {
                let imm = (((instruction & 0xfe000000) as i32 as i64) >> 20) as u64 | ((instruction >> 7) & 0x1f);
                let addr = self.registers[rs1].wrapping_add(imm);
                match funct3{
                    0x00 => {
                        self.store(addr, 8, self.registers[rs2])?;
                    }
                    0x01 => {
                        self.store(addr, 16, self.registers[rs2])?;
                    }
                    0x02 => {
                        self.store(addr, 32, self.registers[rs2])?;
                    }
                    0x03 => {
                        self.store(addr, 64, self.registers[rs2])?;
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //atomics
            0x2f => {
                let funct5 = (funct7 & 0b1111100) >> 2;
                let _aq = (funct7 & 0b0000010) >> 1;
                let _rl = funct7 & 0b0000001; 
                match (funct3, funct5) {
                    (0x2, 0x00) => {
                        let val = self.load(self.registers[rs1], 32)?;
                        self.registers[rd] = val;
                        self.store(self.registers[rs1], 32, val.wrapping_add(self.registers[rs2]))?;
                    }
                    (0x3, 0x00) => {
                        let val: u64 = self.load(self.registers[rs1], 64)?;
                        self.registers[rd] = val;
                        self.store(self.registers[rs1], 64, val.wrapping_add(self.registers[rs2]))?;
                    }
                    (0x2, 0x01) => {
                        let val = self.load(self.registers[rs1], 32)?;
                        self.registers[rd] = val;
                        self.store(self.registers[rs1], 32, self.registers[rs2])?;
                    }
                    (0x3, 0x01) => {
                        let val = self.load(self.registers[rs1], 64)?;
                        self.registers[rd] = val;
                        self.store(self.registers[rs1], 64, self.registers[rs2])?;
                    }
                    _ => {
                        eprintln!("Have not implemented funct3: {:#x} funct5: {:#x}", funct3, funct5);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            0x33 => {
                let shiftamt = ((self.registers[rs2] & 0x3f) as u64) as u32;
                match (funct3, funct7) {
                    //add
                    (0x0, 0x00) => {
                        self.registers[rd] = self.registers[rs1].wrapping_add(self.registers[rs2]);
                    }
                    //mul
                    (0x0, 0x01) => {
                        self.registers[rd] = self.registers[rs1].wrapping_mul(self.registers[rs2]);
                    }
                    //sub
                    (0x0, 0x20) => {
                        self.registers[rd] = self.registers[rs1].wrapping_sub(self.registers[rs2]);
                    }
                    //xor
                    (0x4, 0x00) => {
                        self.registers[rd] = self.registers[rs1] ^ (self.registers[rs2]);
                    }
                    //or
                    (0x6, 0x00) => {
                        self.registers[rd] = self.registers[rs1] | (self.registers[rs2]);
                    }
                    //and
                    (0x7, 0x00) => {
                        self.registers[rd] = self.registers[rs1] & (self.registers[rs2]);
                    }
                    //sll
                    (0x1, 0x00) => {
                        self.registers[rd] = self.registers[rs1].wrapping_shl(shiftamt);
                    }
                    //slr
                    (0x5, 0x00) => {
                        self.registers[rd] = self.registers[rs1].wrapping_shr(shiftamt);
                    }
                    (0x5, 0x01) => {
                        self.registers[rd] = match self.registers[rs2] {
                            0 => {
                                0xffffffff_ffffffff
                            }
                            _ => {
                                let dividend = self.registers[rs1];
                                let divisor = self.registers[rs2];
                                dividend.wrapping_div(divisor)
                            }
                        };  
                    }
                    //sra
                    (0x5, 0x20) => {
                        self.registers[rd] = (self.registers[rs1] as i64).wrapping_shr(shiftamt) as u64;
                    }
                    //slt
                    (0x2, 0x00) => {
                        self.registers[rd] = if (self.registers[rs1] as i64) < (self.registers[rs2] as i64) { 1 } else { 0 };
                    }
                    //sltu
                    (0x3, 0x00) => {
                        self.registers[rd] = if self.registers[rs1] < self.registers[rs2] { 1 } else { 0 };
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //lui
            0x37 => {
                self.registers[rd] = (instruction & 0xfffff000) as i32 as i64 as u64;
            }
            0x3b => {
                let shiftamt = (self.registers[rs2] & 0x1f) as u32;
                match (funct3, funct7) {
                    //addw             
                    (0x0, 0x00) => {
                        self.registers[rd] =
                            self.registers[rs1].wrapping_add(self.registers[rs2]) as i32 as i64 as u64;
                    }
                    //subw
                    (0x0, 0x20) => {
                        self.registers[rd] =((self.registers[rs1].wrapping_sub(self.registers[rs2])) as i32) as i64 as u64;
                    }
                    //sllw
                    (0x1, 0x00) => {
                        self.registers[rd] = ((self.registers[rs1] as u32).wrapping_shl(shiftamt) as i32) as i64 as u64;
                    }
                    //srlw
                    (0x5, 0x00) => {
                        self.registers[rd] = ((self.registers[rs1] as u32).wrapping_shr(shiftamt) as i32) as i64 as u64;
                    }
                    //sraw
                    (0x5, 0x20) => {
                        self.registers[rd] = (self.registers[rs1] as i32).wrapping_shr(shiftamt) as i64 as u64;
                    }
                    // remuw
                    (0x7, 0x01) => {
                        self.registers[rd] = match self.registers[rs2] {
                            0 => self.registers[rs1],
                            _ => {
                                let dividend = self.registers[rs1] as u32;
                                let divisor = self.registers[rs2] as u32;
                                dividend.wrapping_rem(divisor) as u64
                            }
                        };
                    }
                    _ => {
                        println!(
                            "not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}",
                            opcode, funct3, funct7
                        );
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            //branchs
            0x63 => {
                let imm = (((instruction & 0x80000000) as i32 as i64 >> 19) as u64)
                    | ((instruction & 0x80) << 4) 
                    | ((instruction >> 20) & 0x7e0) 
                    | ((instruction >> 7) & 0x1e);
                match funct3 {
                    0x00 => {
                        if self.registers[rs1] == self.registers[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x01 => {
                        if self.registers[rs1] != self.registers[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x04 => {
                        if (self.registers[rs1] as i64) < (self.registers[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x05 => {
                        if (self.registers[rs1] as i64) >= (self.registers[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x06 => {
                        if self.registers[rs1] < self.registers[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x07 => {
                        if self.registers[rs1] >= self.registers[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //jalr
            0x67 => {
                match funct3 {
                    0x00 => {
                        let temp = self.pc;
                        let imm = (((instruction >> 20) as i32) as i64) as u64;
                        self.pc = (self.registers[rs1].wrapping_add(imm)) & !1;
                        self.registers[rd] = temp;
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            //jal 
            0x6f => {
                let imm = (((instruction & 0x80000000) as i32 as i64 >> 11) as u64) 
                    | (instruction & 0xff000) 
                    | ((instruction >> 9) & 0x800) 
                    | ((instruction >> 20) & 0x7fe);
                self.registers[rd] = self.pc;
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            //zicsr
            0x73 => {
                let csr = ((instruction >> 20) & 0xfff) as usize;
                match funct3{
                    0x0 => {
                        match (rs2, funct7) {
                            (0x0, 0x0) => {
                                match self.curr_mode {
                                    Mode::Machine => return Err(Exception::EnvironmentCallFromMMode),
                                    Mode::Supervisor => return Err(Exception::EnvironmentCallFromSMode),
                                    Mode::User => return Err(Exception::EnvironmentCallFromUMode),
                                }
                            }
                            (0x1, 0x0) => {
                                return Err(Exception::Breakpoint)
                            }
                            (0x2, 0x8) => {
                                self.pc = self.load_csr(SEPC);
                                let mode = self.load_csr(SSTATUS) >> 8 & 1;
                                match mode {
                                    1 => self.curr_mode = Mode::Supervisor,
                                    _ => self.curr_mode = Mode::User,
                                };
                                let mut new_sstatus = self.load_csr(SSTATUS);
                                let spie = (self.load_csr(SSTATUS) >> 5) & 1; 
                                if spie == 1 {
                                    new_sstatus |= 1 << 1; 
                                } else {
                                    new_sstatus &= !(1 << 1); 
                                }
                                new_sstatus |= 1 << 5;
                                new_sstatus &= !(1 << 8); 
                                self.store_csr(SSTATUS, new_sstatus);
                            }
                            (0x2, 0x18) => {
                                self.pc = self.load_csr(MEPC);
                                let mode = (self.load_csr(MSTATUS) >> 11) & 0b11;
                                match mode {
                                    2 => self.curr_mode = Mode::Machine,
                                    1 => self.curr_mode = Mode::Supervisor,
                                    _ => self.curr_mode = Mode::User,
                                }
                                let mut new_mstatus = self.load_csr(MSTATUS);
                                let spie = (self.load_csr(MSTATUS) >> 7) & 1; 
                                if spie == 1 {
                                    new_mstatus |= 1 << 3; 
                                } else {
                                    new_mstatus &= !(1 << 3); 
                                }
                                new_mstatus |= 1 << 7;
                                new_mstatus &= !(0b11 << 11);
                                self.store_csr(MSTATUS, new_mstatus);
                            }
                            _ => {
                                println!(
                                    "not implemented yet: opcode {:#x} funct3 {:#x} funct7 {:#x}",
                                    opcode, funct3, funct7
                                );
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x1 => {
                        let val = self.load_csr(csr);
                        self.store_csr(csr, self.registers[rs1]);
                        self.registers[rd] = val;
                    }
                    0x2 => {
                        let val = self.load_csr(csr);
                        self.store_csr(csr, val | self.registers[rs1]);
                        self.registers[rd] = val;
                    }
                    0x3 => {
                        let val = self.load_csr(csr);
                        self.store_csr(csr, val & (!self.registers[rs1]));
                        self.registers[rd] = val;
                    }
                    0x5 => {
                        let imm = rs1 as u64;
                        self.registers[rd] = self.load_csr(csr);
                        self.store_csr(csr, imm);
                    }
                    0x6 => {
                        let imm = rs1 as u64;
                        let t = self.load_csr(csr);
                        self.store_csr(csr, t | imm);
                        self.registers[rd] = t;
                    }
                    0x7 => {
                        let imm = rs1 as u64;
                        let t = self.load_csr(csr);
                        self.store_csr(csr, t & (!imm));
                        self.registers[rd] = t;
                    }
                    _ => {
                        eprintln!("Have not implemented opcode: {:#x} funct3: {:#x}", opcode, funct3);
                        return Err(Exception::IllegalInstruction)
                    }
                }
            }
            _ =>{
                dbg!("Not done");
                eprintln!("Have not implemented opcode: {:#x}", opcode);
                return Err(Exception::IllegalInstruction)
            }
        }
        self.registers[0] = 0;
        Ok(())
    }

    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.registers[i],
                    i + 1,
                    abi[i + 1],
                    self.registers[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.registers[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.registers[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    pub fn dump_csrs(&self) {
        let output = format!(
            "{}\n{}",
            format!(
                "mstatus={:>#18x} mtvec={:>#18x} mepc={:>#18x} mcause={:>#18x}",
                self.load_csr(MSTATUS),
                self.load_csr(MTVEC),
                self.load_csr(MEPC),
                self.load_csr(MCAUSE),
            ),
            format!(
                "sstatus={:>#18x} stvec={:>#18x} sepc={:>#18x} scause={:>#18x}",
                self.load_csr(SSTATUS),
                self.load_csr(STVEC),
                self.load_csr(SEPC),
                self.load_csr(SCAUSE),
            ),
        );
        println!("{}", output);
    }
}

