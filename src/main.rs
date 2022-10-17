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
	}
	
	// Some mock data
	todo_list.push(todo::TodoItem{title: "Test 1".to_string(), description: "Test description 1".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 2".to_string(), description: "Test description 2".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 3".to_string(), description: "Test description 3".to_string(), completed: false});
	
	// Show the welcome message
	clear!();
	helpers::print_centre("Hello, welcome to ToDoApp version 3!", &term_size);
	helpers::print_div(helpers::DividerType::Double, &term_size);
	helpers::print_centre("... it's like before, but this time in Rust 🦀!", &term_size);
	helpers::print_div(helpers::DividerType::Star, &term_size);
	helpers::pause();

	// Print all todo items
	clear!();
	println!("You currently have {} to-do items.", todo_list.len());
	helpers::print_div(helpers::DividerType::Double, &term_size);
	for item in todo_list {
		println!("Title: {}, Descritpion: {}, Completed: {}", item.title, item.description, item.completed);
		helpers::print_div(helpers::DividerType::Single, &term_size);
	}
}
