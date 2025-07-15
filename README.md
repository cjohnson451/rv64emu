# rvemu

A basic RV64I (RISC-V 64-bit) emulator written in Rust.

## Features

- Loads and executes RV64I binary files.
- Emulates CPU, DRAM, bus, and basic interrupt/trap handling.
- Prints register and CSR state after execution.
- Includes a Python script to convert hex instruction strings to binary files.

## Usage

1. **Build the emulator:**

   ```
   cargo build --release
   ```

2. **Run a binary:**

   ```
   ./target/release/rvemu <your_binary_file.bin>
   ```

   Optionally, add `--no-trap` to exit on the first trap.

3. **Convert hex to binary (optional):**

   Use `hex_to_bin_converter.py` to create a `.bin` file from a hex string:

   ```
   python3 hex_to_bin_converter.py
   ```

   This will generate `comprehensive_test.bin` (edit the script to change the hex or output file).

## Need to merge and push

- UART (serial) support
- MMU (Memory Management Unit) emulation

## Inspired by and with reference to 

- rvemu
- Rare