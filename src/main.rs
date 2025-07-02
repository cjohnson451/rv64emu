use std::io::{self, Read};
use std::env;
use std::fs::File;

mod cpu;
mod dram; 
mod bus;
mod trap;
use crate::cpu::*;
use crate::trap::*;


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        eprintln!("Usage: rvemu <filename>");
        std::process::exit(1);
    }
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut code: Vec<u8> = Vec::new();
    file.read_to_end(&mut code)?;
    let mut cpu = Cpu::new(code);
    loop{
            let instruction = match cpu.fetch(){
                Ok(instuction) => instuction,
                Err(exception) => {
                    exception.handle_trap(&mut cpu);
                    if exception.is_fatal() {
                        break;
                    }
                    0 
                }
            };
            
            cpu.pc += 4;

            match cpu.execute(instruction){
                Ok(_) => {},
                Err(exception) => {
                    exception.handle_trap(&mut cpu);
                    if exception.is_fatal() {
                        break;
                    }
                }
            }

            if cpu.pc == 0{
                break;
            }
    }
    cpu.dump_registers();
    Ok(())
}

