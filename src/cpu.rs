use crate::bus::*;
use crate::dram::{DRAM_SIZE, DRAM_BASE};

pub struct Cpu{
    pub registers: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu{
    pub fn new(binary: Vec<u8>) -> Self{
        let mut regs = [0; 32];
        regs[2] = DRAM_BASE + DRAM_SIZE;
        Self {
            registers: regs,
            pc: DRAM_BASE,
            bus: Bus::new(binary),
        }
    }   

    pub fn fetch(&mut self) -> Result<u64, ()> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(_e) => Err(()),
        }
    }

    pub fn execute(&mut self, instruction: u64) -> Result<(), ()>{
        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let funct3 = ((instruction >> 12) & 0x07) as usize;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;

        self.registers[0] = 0;
        // Keep adding opcodes. Maybe add load and store fns for cpu. 
        match opcode{
            0x03 =>{
                let imm = ((instruction as i32 as i64) >> 20) as u64;
                let addr = self.registers[rs1].wrapping_add(imm);
                match funct3{
                    0x00 => {
                        let data = self.bus.load(addr, 8)? as i8 as i64 as u64;
                        self.registers[rd] = data;
                    }
                    _ => {}
                }
            }
            0x13 => {
                let imm = ((instruction as i32 as i64) >> 20) as u64;
                self.registers[rd] = self.registers[rs1].wrapping_add(imm);
            }
            0x33 => {
                self.registers[rd] = self.registers[rs1].wrapping_add(self.registers[rs2]);
            }
            _ =>{
                dbg!("Not done");
                return Err(())
            }
        }
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
}