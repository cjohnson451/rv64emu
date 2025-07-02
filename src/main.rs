use std::io::{self, Read};
use std::env;
use std::fs::File;

mod cpu;
mod dram; 
mod bus;
mod trap;
mod plic;
mod clint;
use crate::cpu::*;
use crate::trap::*;


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3{
        eprintln!("Usage: rvemu <filename> [--no-trap]");
        std::process::exit(1);
    }
    let filename = &args[1];
    let no_trap = args.len() == 3 && args[2] == "--no-trap";
    let mut file = File::open(filename)?;
    let mut code: Vec<u8> = Vec::new();
    file.read_to_end(&mut code)?;
    let mut cpu = Cpu::new(code);
    loop{
            let instruction = match cpu.fetch(){
                Ok(instruction) => instruction,
                Err(exception) => {
                    if no_trap {
                        break;
                    }
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
                    if no_trap {
                        break;
                    }
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
    println!("-----------------------------------------------------------------------------------------------------------");
    cpu.dump_csrs();
    Ok(())
}

