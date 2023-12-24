// $t@$h
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    User,
    Kernel,
    Hypervisor,
    UEFI,
    Off,
}

enum CommandResult {
    Success,
    NotVerified,
    UnknownCommand,
}

type InstructionHandler = fn();

struct State {
    mode: Mode,
}

impl State {
    fn new() -> Self {
        State {
            mode: Mode::Off,
        }
    }

    fn change_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }
	
	fn current_mode(&self) -> Mode {
        self.mode
    }
}

fn process_command(command: &str, state: &mut State) -> CommandResult {
    match command {
		"shutdown" => {
			println!("ACPI shutdown received.");
			println!("Shredding sensitive data.");
			println!("Encrypting disk and memory.");
            println!("System shutting down...");
            state.change_mode(Mode::Off);
            CommandResult::Success
        },
        "load_hypervisor" => {
            let is_vm = IS_VERIFIED_VM.lock().unwrap();
            if !*is_vm {
                println!("Hypervisor not verified. Aborting.");
                CommandResult::NotVerified
            } else {
                println!("Hypervisor loaded.");
                state.change_mode(Mode::Hypervisor);
                CommandResult::Success
            }
        },
        "load_kernel" => {
            let is_os = IS_VERIFIED_OS.lock().unwrap();
            if !*is_os {
                println!("Kernel not verified. Aborting.");
                CommandResult::NotVerified
            } else {
                println!("Kernel loaded.");
                state.change_mode(Mode::Kernel);
                CommandResult::Success
            }
        },
        "load_application" => {
            let is_ap = IS_VERIFIED_AP.lock().unwrap();
            if !*is_ap {
                println!("Application not verified. Aborting.");
                CommandResult::NotVerified
            } else {
                println!("Application loaded.");
                state.change_mode(Mode::User);
                CommandResult::Success
            }
        },
        _ => CommandResult::UnknownCommand,
    }
}

fn provide_hint(mode: Mode) {
    match mode {
        Mode::Off => println!("Hint: Type 'powerup' to start the board"),
        Mode::UEFI => println!("Hint: Type 'load_hypervisor' to load Hypervisor mode"),
        Mode::Hypervisor => println!("Hint: Type 'load_kernel' to load the Kernel mode"),
        Mode::Kernel => println!("Hint: Type 'start_user_space' to start user space applications"),
        Mode::User => println!("Hint: Execute user-level instructions like 'ADD', 'SUB', etc."),
    }
}

fn print_instructions_list(instruction_map: &HashMap<&str, InstructionHandler>) {
    println!("Available Instructions:");
    for instruction in instruction_map.keys() {
        println!(" {}", instruction);
    }
}

lazy_static! {
	static ref IS_VERIFIED_BL: Mutex<bool> = Mutex::new(false);
	static ref IS_VERIFIED_VM: Mutex<bool> = Mutex::new(false);
	static ref IS_VERIFIED_OS: Mutex<bool> = Mutex::new(false);
	static ref IS_VERIFIED_FS: Mutex<bool> = Mutex::new(false);
	static ref IS_VERIFIED_AP: Mutex<bool> = Mutex::new(false);
}

fn get_prompt_color(mode: Mode) -> &'static str {
    match mode {
        Mode::UEFI => "\x1b[38;5;14m",
        Mode::Hypervisor => "\x1b[38;5;13m",
        Mode::Kernel => "\x1b[38;5;10m",
        Mode::User => "\x1b[38;5;11m",
        Mode::Off => "\x1b[0m",
    }
}

fn main() {
    let ansi_color_code = "\x1b[38;5;197m"; // 197 is a close approximation for #FF6699 in the 256-color palette
    let reset_code = "\x1b[0m"; // Resets the color
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
	
	let mut state = State::new();

	fn change_to_hypervisor_mode(mode: &mut Mode) {
       *mode = Mode::Hypervisor;
    }
	
	fn change_to_kernel_mode(mode: &mut Mode) {
       *mode = Mode::Kernel;
    }
	
	fn change_to_user_mode(mode: &mut Mode) {
       *mode = Mode::User;
    }
	
	fn verify_bootloader() {
		println!("Verified Bootloader");
		let mut is_bl = IS_VERIFIED_BL.lock().unwrap();
		*is_bl = true;
	}

	fn verify_hypervisor() {
		println!("Verified Hypervisor");
		let mut is_vm = IS_VERIFIED_VM.lock().unwrap();
		*is_vm = true;
	}
	
	fn verify_kernel() {
		println!("Verified Guest OS kernel");
		let mut is_os = IS_VERIFIED_OS.lock().unwrap();
		*is_os = true;
	}
	
	fn verify_filesystem() {
		println!("Verified filesystem");
		let mut is_fs = IS_VERIFIED_FS.lock().unwrap();
		*is_fs = true;
	}
	
	fn verify_application() {
		println!("Verified Application");
		let mut is_ap = IS_VERIFIED_AP.lock().unwrap();
		*is_ap = true;
	}

    fn init_full_hw() { println!("Hypervisor managed hardware"); }
    fn start_user_space() { println!("User space started"); }

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
		    ("verify_bootloader", verify_bootloader as InstructionHandler, Mode::UEFI),
        ("verify_hypervisor", verify_hypervisor as InstructionHandler, Mode::UEFI),
        ("init_full_hw", init_full_hw as InstructionHandler, Mode::Hypervisor),
	    	("verify_kernel", verify_kernel as InstructionHandler, Mode::Hypervisor),
        ("verify_filesystem", verify_filesystem as InstructionHandler, Mode::Kernel),
        ("verify_application", verify_application as InstructionHandler, Mode::Kernel),
        ("start_user_space", start_user_space as InstructionHandler, Mode::Kernel),
        // TODOs: data at rest and in motion encryption logic
        // TODOs in Mode::User
    ];
    
    for &(inst, handler, mode) in &instructions {
        instruction_map.insert(inst, handler);
        instruction_modes.insert(inst, mode);
    }

    let mut state = State::new();
    std::io::stdout().flush().unwrap();
    'shell: loop {
        let prompt_color = get_prompt_color(mode);
        let prompt = format!("{}{:?}>>{}", prompt_color.trim(), mode, reset_code.trim());
		match rl.readline(&prompt) {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.is_empty() { continue; }
				let cmd = parts[0];

				match process_command(cmd, &mut state) {
					CommandResult::Success => {
                                            mode = state.current_mode(); // Update mode only on success
                                            continue 'shell;
                                        },
                                        CommandResult::NotVerified => continue 'shell,
					CommandResult::UnknownCommand => {
						match cmd {
							"hint" => provide_hint(mode),
							"instructions" => print_instructions_list(&instruction_map),
							"powerup" => {
								mode = match mode {
									Mode::Off => Mode::UEFI,
									Mode::UEFI => Mode::Hypervisor,
									Mode::Hypervisor => Mode::Kernel,
									Mode::Kernel => Mode::User,
									Mode::User => Mode::User,
								};
								println!("Switched to {:?} mode", mode);
							},
							"exit" => break,
							_ => {
								if let Some(&required_mode) = instruction_modes.get(cmd) {
									if required_mode == mode {
										execute_instruction(cmd, &instruction_map, &instruction_modes, mode);
									} else {
										println!("Cannot access '{}' in {:?} mode", cmd, mode);
									}
								} else {
									println!("Unknown instruction: '{}'", cmd);
								}
							},
						}
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
