use std::io::{stdin};//,stdout,Write};
mod parser;

fn main() {
    let mut editing = true;
    let mut s = String::new(); // Stores the inputted command string
    let mut dot = 0;
    
    while editing {
	// Reset the command string
	s = "".to_string();

	// Read the command
	stdin().read_line(&mut s).expect("Failed to read line");

	// Remove the trailing newline
	s = s.trim().to_string();

	// Parse the command string
	let commands = parser::parse(&s);

	for cmd in commands {
	    match cmd {
		parser::Actions::QUIT => {
		    println!("Quit");
		    editing = false;
		    break;
		},
		parser::Actions::PRINT => println!("Print"),
		parser::Actions::MOVE {line_num} => {
		    println!("Move");
		},
	    }
	}
    }
}
