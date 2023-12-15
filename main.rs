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
}

type InstructionHandler = fn();

fn main() {
    let mut rl = Editor::<()>::new();
    let mut mode = Mode::UEFI;
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
    fn init_uefi() { println!("Initialized UEFI firmware mode"); }
    fn load_hypervisor() { println!("Loaded Hypervisor in Hypervisor mode"); }
    fn load_kernel() { println!("Kernel loaded in Kernel mode"); }
    fn start_user_space() { println!("User space applications started in User mode"); }
    fn verify_signature() { println!("Verified digital signature"); }

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
        ("init_uefi", init_uefi as InstructionHandler, Mode::UEFI),
        ("load_hypervisor", load_hypervisor as InstructionHandler, Mode::Hypervisor),
        ("load_kernel", load_kernel as InstructionHandler, Mode::Kernel),
        ("start_user_space", start_user_space as InstructionHandler, Mode::User),
        ("verify_signature", verify_signature as InstructionHandler, Mode::UEFI),
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
                    "switch_mode" => {
                        mode = match mode {
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
