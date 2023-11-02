use std::collections::VecDeque;
use std::io::{stdin, stdout, Read, Write};

const C_MEM_SIZE: usize = 30000;
const C_CELL_MAX: u8 = 255;

pub fn evaluate(scode: String) {
    let code = scode.as_bytes();

    let mut jumps: VecDeque<usize> = VecDeque::new();
    let mut code_ptr: usize = 0;
    let mut ptr: usize = 0;
    // TODO: Make mem size variable
    let mut cells = [0; C_MEM_SIZE];

    loop {
        let c = code[code_ptr];
        match c as char {
            '[' => jumps.push_front(code_ptr),
            ']' => match cells[ptr] {
                0 => (),
                _ => match jumps.pop_front() {
                    Some(jump) => {
                        code_ptr = jump;
                        continue;
                    }
                    None => panic!("Expected popping some jump point!"),
                },
            },
            '+' => match cells[ptr] {
                C_CELL_MAX => cells[ptr] = 0,
                _ => cells[ptr] += 1,
            },
            '-' => match cells[ptr] {
                0 => cells[ptr] = C_CELL_MAX,
                _ => cells[ptr] -= 1,
            },
            '.' => print!("{}", cells[ptr] as char),
            '<' => match ptr {
                0 => ptr = C_MEM_SIZE - 1,
                _ => ptr -= 1,
            },
            '>' => match ptr {
                p if p >= (C_MEM_SIZE - 1) => ptr = 0,
                _ => ptr += 1,
            },
            ',' => {
                stdout().flush().unwrap();
                match stdin().bytes().next() {
                    Some(c) => match c {
                        Ok(c) => cells[ptr] = c,
                        Err(e) => panic!("{}", e),
                    },
                    None => panic!("Expected some value from stdin!"),
                }
            }
            _ => (),
        }
        code_ptr += 1;
        // Loop exit condition
        if code_ptr >= code.len() {
            break;
        }
    }
}
