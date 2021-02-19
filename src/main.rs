use std::io;

#[derive(Copy, Clone)]
enum MachineState {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn machine_cycle(state: &MachineState, c: char) -> (Option<char>, MachineState) {
    use self::MachineState::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn main() {
    let mut state = MachineState::Normal;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();

    let mut processed_string = String::new();
    for character in input.chars() {
        let (output, new_state) = machine_cycle(&mut state, character);
        if let Some(c) = output { processed_string.push(c) };
        state = new_state;
    }

    println!("Processed string: {}", processed_string);
}
