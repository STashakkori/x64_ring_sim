// $t@$h
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    User,
    Kernel,
    Hypervisor,
    UEFI,
    Off,
}

type InstructionHandler = fn();

fn provide_hint(mode: Mode) {
    match mode {
        Mode::Off => println!("Hint: Type 'init_uefi' to start the board"),
        Mode::UEFI => println!("Hint: Type 'load_hypervisor' to load Hypervisor mode"),
        Mode::Hypervisor => println!("Hint: Type 'load_kernel' to load the Kernel mode"),
        Mode::Kernel => println!("Hint: Type 'start_user_space' to start user space applications"),
        Mode::User => println!("Hint: Execute user-level instructions like 'ADD', 'SUB', etc."),
    }
}

fn print_instructions_list(instruction_map: &HashMap<&str, InstructionHandler>) {
    println!("Available Instructions:");
    for instruction in instruction_map.keys() {
        println!("- {}", instruction);
    }
}

fn main() {
    let ansi_color_code = "\x1b[38;5;197m"; // 197 is a close approximation for #FF6699 in the 256-color palette
    let reset_code = "\x1b[0m"; // Resets the color
    println!("  Secure booter 0: A serious game");
    println!("    Copyright QVLX LLC 2023");
    println!("    All rights reserved.\n");
    println!("{}  Secure booter 0: A serious game{}", ansi_color_code, reset_code);
    println!("{}    Copyright QVLX LLC 2023{}", ansi_color_code, reset_code);
    println!("{}    All rights reserved.\n{}", ansi_color_code, reset_code);
    println!("** x8664 Edition **");
    println!("type powerup once to start");
    println!("type instructions for superset");
    let mut rl = Editor::<()>::new();
    let mut mode = Mode::Off;
    let mut instruction_map: HashMap<&str, InstructionHandler> = HashMap::new();
    let mut instruction_modes: HashMap<&str, Mode> = HashMap::new();

    // x86/64 Instruction Handlers
    fn add_handler() { println!("Executed ADD instruction"); }
    fn sub_handler() { println!("Executed SUB instruction"); }
    fn mul_handler() { println!("Executed MUL instruction"); }
    fn div_handler() { println!("Executed DIV instruction"); }
    fn xor_handler() { println!("Executed XOR instruction"); }
    fn and_handler() { println!("Executed AND instruction"); }
    fn or_handler() { println!("Executed OR instruction"); }
    fn mov_handler() { println!("Executed MOV instruction"); }
    fn jmp_handler() { println!("Executed JMP instruction"); }
    fn cmp_handler() { println!("Executed CMP instruction"); }
    fn inc_handler() { println!("Executed INC instruction"); }
    fn dec_handler() { println!("Executed DEC instruction"); }
    fn push_handler() { println!("Executed PUSH instruction"); }
    fn pop_handler() { println!("Executed POP instruction"); }
    fn call_handler() { println!("Executed CALL instruction"); }
    fn ret_handler() { println!("Executed RET instruction"); }
    fn nop_handler() { println!("Executed NOP instruction"); }
    fn lea_handler() { println!("Executed LEA instruction"); }

    // x86/64 System-level Instruction Handlers with Secure Boot
    fn init_initial_hw() { println!("Initialized UEFI firmware mode"); }
    fn verify_hypervisor() { println!("Verified Hypervisor"); }
    fn load_hypervisor() { println!("Loaded Hypervisor"); }
    fn init_full_hw() { println!("Hypervisor managed hardware"); }
    fn verify_bootloader() { println!("Verified bootloader"); }
    fn load_kernel() { println!("Bootstrapped kernel"); }
    fn verify_filesystem() { println!("Verified filesystem"); }
    fn start_user_space() { println!("User space started"); }
    fn verify_application() { println!("Verified application"); }
    fn load_application() { println!("Application loaded in userspace"); }

    // Instructions
    let instructions = [
        ("ADD", add_handler as InstructionHandler, Mode::User),
        ("SUB", sub_handler as InstructionHandler, Mode::User),
        ("MUL", mul_handler as InstructionHandler, Mode::User),
        ("DIV", div_handler as InstructionHandler, Mode::User),
        ("XOR", xor_handler as InstructionHandler, Mode::User),
        ("AND", and_handler as InstructionHandler, Mode::User),
        ("OR", or_handler as InstructionHandler, Mode::User),
        ("MOV", mov_handler as InstructionHandler, Mode::User),
        ("JMP", jmp_handler as InstructionHandler, Mode::User),
        ("CMP", cmp_handler as InstructionHandler, Mode::User),
        ("INC", inc_handler as InstructionHandler, Mode::User),
        ("DEC", dec_handler as InstructionHandler, Mode::User),
        ("PUSH", push_handler as InstructionHandler, Mode::User),
        ("POP", pop_handler as InstructionHandler, Mode::User),
        ("CALL", call_handler as InstructionHandler, Mode::User),
        ("RET", ret_handler as InstructionHandler, Mode::User),
        ("NOP", nop_handler as InstructionHandler, Mode::User),
        ("LEA", lea_handler as InstructionHandler, Mode::User),

        // System instructions (ish). I need to rework this
        ("init_initial_hw", init_initial_hw as InstructionHandler, Mode::UEFI),
        ("verify_hypervisor", verify_hypervisor as InstructionHandler, Mode::UEFI),
        ("load_hypervisor",  load_hypervisor as InstructionHandler, Mode::Hypervisor),
        ("init_full_hw", init_full_hw as InstructionHandler, Mode::Hypervisor),
        ("verify_bootloader", verify_bootloader as InstructionHandler, Mode::Hypervisor),
        ("load_kernel", load_kernel as InstructionHandler, Mode::Hypervisor),
        ("verify_filesystem", verify_filesystem as InstructionHandler, Mode::Kernel),
        ("verify_application", verify_application as InstructionHandler, Mode::Kernel),
        ("start_user_space", start_user_space as InstructionHandler, Mode::Kernel),
        ("load_application", load_application as InstructionHandler, Mode::Kernel),
        // in Mode::User now
    ];
    
    for &(inst, handler, mode) in &instructions {
        instruction_map.insert(inst, handler);
        instruction_modes.insert(inst, mode);
    }

    loop {
        let prompt = format!("{:?}>> ", mode);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() { continue; }

                match parts[0] {
                    "hint" => provide_hint(mode),
                    "instructions" => print_instructions_list(&instruction_map),
                    "powerup" => {
                        mode = match mode {
                            Mode::Off => Mode::UEFI,
                            Mode::UEFI => Mode::Hypervisor,
                            Mode::Hypervisor => Mode::Kernel,
                            Mode::Kernel => Mode::User,
                            Mode::User => {
                                println!("Already in User mode");
                                Mode::User
                            },
                        };
                        println!("Switched to {:?} mode", mode);
                    },
                    "exit" => break,
                    _ => {
                        let inst = parts[0];
                        execute_instruction(inst, &instruction_map, &instruction_modes, mode);
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => println!("Error: {:?}", err),
        }
    }
}

fn execute_instruction(instruction: &str, instruction_map: &HashMap<&str, InstructionHandler>, instruction_modes: &HashMap<&str, Mode>, mode: Mode) {
    if let Some(&handler) = instruction_map.get(instruction) {
        if let Some(&required_mode) = instruction_modes.get(instruction) {
            if required_mode == mode {
                handler();
            } else {
                println!("Error: '{}' cannot be executed in current mode", instruction);
            }
        } else {
            println!("Unknown instruction mode requirement");
        }
    } else {
        println!("Unknown instruction");
    }
}
