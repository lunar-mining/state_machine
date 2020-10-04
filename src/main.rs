//#[derive(Debug)]
use std::io;

enum State {
    State0,
    State1,
}

fn main() {
    let mut state = State::State0;
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

        state = state_transition(input, state);
    }
}

fn state_transition(input: u32, state: State) -> State {
    println!("Input is {}", input);
    match input {
        0 => {
            match state {
                State::State0 => State::State0,
                State::State1 => State::State1
            }
        }
        1 => {
            match state {
                State::State0 => State::State1,
                State::State1 => State::State0
            }
        }
        _ => unreachable!(),
    };
    return state;
}
