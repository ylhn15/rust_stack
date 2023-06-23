use std::io;

static mut POINTER: usize = 0;
const STACK_SIZE: usize = 100;
static mut STACK: [i32; STACK_SIZE] = [0; STACK_SIZE];

fn main() {
    loop {
        let mut command = String::new();
        println!("waiting for instruction");
        io::stdin()
            .read_line(&mut command)
            .expect("error: unable to read user input");
        match command
            .split_whitespace()
            .nth(0)
            .expect("Wrong command format")
        {
            "exit" => return,
            "add" => add(),
            "sub" => subtract(),
            "mul" => multiply(),
            "div" => divide(),
            "mod" => modulo(),
            "print" => print(),
            "inspect" => inspect_stack(),
            "push" => {
                let input_value: i32 = command
                    .split_whitespace()
                    .nth(1)
                    .expect("Wrong command format")
                    .parse()
                    .expect("Cant parse input");
                push(input_value);
            }
            &_ => println!("unknown command: {}", command),
        }
    }
}

fn inspect_stack() {
    unsafe {
        println!("COUNTER POSITION = {}", POINTER);
        println!("STACK START");
        println!("==============");
        for value in 0..POINTER {
            println!("stack[{}] = {}", value, STACK[value]);
        }
        println!("==============");
        println!("STACK END");
        println!("\n");
    }
}

fn push(value: i32) {
    unsafe {
        STACK[POINTER] = value;
        POINTER += 1;
    }
}

fn pop() -> i32 {
    unsafe {
        if POINTER > 0 {
            POINTER -= 1;
        } else {
            println!("Cannot perform function on empty stack");
        }
        STACK[POINTER]
    }
}

fn print() {
    unsafe {
        let head = POINTER - 1;
        println!("stack[{}] = {}", head, STACK[head]);
    }
}

fn add() {
    let value = pop() + pop();
    push(value);
}

fn multiply() {
    let value = pop() * pop();
    push(value);
}

fn subtract() {
    let second = pop();
    let first = pop();
    push(first - second);
}

fn divide() {
    let second = pop();
    let first = pop();
    push(first / second);
}

fn modulo() {
    let second = pop();
    let first = pop();
    push(first % second);
}
