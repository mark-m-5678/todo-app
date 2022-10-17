use std::io;

mod helpers;
mod todo;
extern crate termsize; // <-- Just makes it easier to get the terminal window size on any OS

fn main() {
	// Init
	let mut todo_list = Vec::new();
	let term_size = if let Some(size) = termsize::get() {
		helpers::TerminalDimensions{width: size.cols, height: size.rows}
	} else {
		helpers::TerminalDimensions{width: 100, height: 20}
	};
	let options = vec!["Add a new item", "Edit an existing item", "Delete an item", "Quit"];
	
	// Some mock data
	todo_list.push(todo::TodoItem{title: "Test 1".to_string(), description: "Test description 1".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 2".to_string(), description: "Test description 2".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 3".to_string(), description: "Test description 3".to_string(), completed: false});
	
	// Show the welcome message
	print_welcome_message(&term_size);

	// Print all todo items
	print_todo_list(&todo_list, &term_size);
	
	print_options(&options);

	let mut user_input = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut user_input).unwrap();
	println!("You entered: {}", user_input);
	// TODO: Finish this
}

fn print_welcome_message(term_size: &helpers::TerminalDimensions) {
	clear!();
	helpers::print_centre("Hello, welcome to ToDoApp version 3!", &term_size);
	helpers::print_div(helpers::DividerType::Double, &term_size);
	helpers::print_centre("... it's like before, but this time in Rust 🦀!", &term_size);
	helpers::print_div(helpers::DividerType::Star, &term_size);
	helpers::pause();
}

fn print_options(options: &Vec<&str>) {
	print!("Please select from the following options:\n");

	for (i, option) in options.iter().enumerate() {
		println!("{}. {}", i + 1, option);
	}
}

fn print_todo_list(todo_list: &Vec<todo::TodoItem>, term_size: &helpers::TerminalDimensions) {
	clear!();
	println!("You currently have {} to-do items.", todo_list.len());
	helpers::print_div(helpers::DividerType::Double, &term_size);
	for item in todo_list {
		println!("Title: {}, Descritpion: {}, Completed: {}", item.title, item.description, item.completed);
		helpers::print_div(helpers::DividerType::Single, &term_size);
	}
}

fn add_item(todo_list: &mut Vec<todo::TodoItem>, title: String, description: String) {
	let new_item = todo::TodoItem{title, description, completed: false};
	todo_list.push(new_item);
}