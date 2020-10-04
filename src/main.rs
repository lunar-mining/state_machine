//#[derive(Debug)]
use std::io;

enum State {
    State0,
    State1,
}

fn main() {
    loop {
        println!("Enter 0 or 1");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        state_transition(input);
    }
}

// TODO: after first loop set CURRENT STATE to PREVIOUS END STATE
// NOT reset each time to input

fn state_transition(input: u32) {
    let current_state = input;
    println!("Current state is {}", input);
    match input {
        0 => {
            if current_state == 0 {
                println!("Moved from 0 to 0");
                State::State0
            } else {
                println!("Moved from 1 to 1");
                State::State1
            }
        }
        1 => {
            if current_state == 1 {
                println!("Moved from 1 to 0");
                State::State0
            } else {
                println!("Moved from 0 to 1");
                State::State1
            }
        }
        _ => unreachable!(),
    };
}
