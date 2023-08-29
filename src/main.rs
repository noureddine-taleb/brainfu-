use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod tests;
fn main() {
    if env::args().len() < 2 {
        panic!("no file provided");
    }

    let source = read_file(env::args().into_iter().nth(1).unwrap());
    execute(source.as_bytes());
}

fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("can't open the given file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("can't open the given file");
    buf
}

fn __execute(code: &[u8], memory: &mut [u8; 30000], dp: &mut u32, pc: &mut u32) -> u32 {
    let brace_map = build_brace_map(code);
    let mut retired = 0;

    while *pc < code.len() as u32 {
        let opcode = code[*pc as usize] as char;
        match opcode {
            '>' => {
                *dp += 1;
                *pc += 1;
                retired += 1;
            }
            '<' => {
                *dp -= 1;
                *pc += 1;
                retired += 1;
            }
            '+' => {
                memory[*dp as usize] = memory[*dp as usize].overflowing_add(1).0;
                *pc += 1;
                retired += 1;
            }
            '-' => {
                memory[*dp as usize] = memory[*dp as usize].overflowing_sub(1).0;
                *pc += 1;
                retired += 1;
            }
            '.' => {
                println!("{:x}", memory[*dp as usize]);
                *pc += 1;
                retired += 1;
            }
            ',' => {
                let mut buf = [0_u8; 1];
                let count = std::io::stdin().read(&mut buf).expect("can't read from stdin");
                if count != 1 {
                    panic!("can't read from stdin");
                }
                memory[*dp as usize] = buf[0];
                *pc += 1;
                retired += 1;
            }
            '[' => {
                if memory[*dp as usize] == 0 {
                    *pc = brace_map.get(&*pc).unwrap() + 1;
                } else {
                    *pc += 1;
                }
                retired += 1;
            }
            ']' => {
                if memory[*dp as usize] != 0 {
                    *pc = brace_map.get(&*pc).unwrap() + 1;
                } else {
                    *pc += 1;
                }
                retired += 1;
            }
    
            _ => {
                *pc += 1;
            }
        }
    }
    retired
}

fn execute(code: &[u8]) {
    // machine state
    let mut memory = [0_u8; 30000];
    let mut dp = 0_u32;
    let mut pc = 0_u32;

    __execute(code, &mut memory, &mut dp, &mut pc);
}

fn build_brace_map(source: &[u8]) -> HashMap<u32, u32> {
    let mut v: Vec<u32> = vec![];
    let mut map = HashMap::<u32, u32>::new();
    for i in 0..source.len() {
        let opcode = source[i];
        match char::from_u32(opcode as u32).unwrap() {
            '[' => {
                v.push(i as u32);
            }
            ']' => {
                let start = v.pop().expect("braces are mismatched");
                map.insert(start, i as u32);
                map.insert(i as u32, start);
            }
            _ => {}
        }
    }

    assert!(v.len() == 0, "braces are mismatched");
    map
}

