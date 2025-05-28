use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: jgs <filename.jgs>");
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read file");
    let commands = tokenize(&source);
    run(commands);
}

fn tokenize(source: &str) -> Vec<char> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = source.chars().collect();
    while i < chars.len() {
        let current: String = chars[i..].iter().collect();

        // 🔽 最長一致順に並べ替えたバージョン
        if current.starts_with("じゃじゃが"){
            tokens.push('#');
            i += 5;
        } else if current.starts_with("じゃがが") {
            tokens.push('[');
            i += 4;
        } else if current.starts_with("ががじゃ") {
            tokens.push(']');
            i += 4;
        } else if current.starts_with("じゃじゃ") {
            tokens.push('.');
            i += 4;
        } else if current.starts_with("がじゃ") {
            tokens.push('-');
            i += 3;
        } else if current.starts_with("がが") {
            tokens.push(',');
            i += 2;
        } else if current.starts_with("じゃが") {
            tokens.push('+');
            i += 3;
        } else if current.starts_with("じゃ") {
            tokens.push('>');
            i += 2;
        } else if current.starts_with("が") {
            tokens.push('<');
            i += 1;
        } else {
            i += 1;
        };
        }
    tokens
    }

fn run(commands: Vec<char>) {
    let mut tape = vec![0u8; 30000];
    let mut ptr = 0usize;
    let mut pc = 0usize;
    let mut loop_stack = Vec::new();

    while pc < commands.len() {
        match commands[pc] {
            '>' => ptr += 1,
            '<' => {
                if ptr == 0 {
                    ptr = tape.len() - 1;
                } else {
                    ptr -= 1;
                }
            }
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => {
                print!("{}", tape[ptr] as char);
                io::stdout().flush().unwrap();
            }
            ',' => {
                let mut buffer = [0];
                io::stdin().read_exact(&mut buffer).unwrap();
                tape[ptr] = buffer[0];
            }
            '[' => {
                if tape[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        pc += 1;
                        if pc >= commands.len() {
                            panic!("Unmatched '['");
                        }
                        if commands[pc] == '[' {
                            depth += 1;
                        } else if commands[pc] == ']' {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    pc = *loop_stack.last().expect("Unmatched ']'");
                } else {
                    loop_stack.pop();
                }
            }
            '#' => {
                println!("{}", tape[ptr]);
                io::stdout().flush().unwrap();
            }
            _ => {}
        }
        pc += 1;
    }
}