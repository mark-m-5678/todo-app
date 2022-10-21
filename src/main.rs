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
	
	let options = vec!["Add a new item", "Toggle complete", "Edit an existing item", "Delete an item", "Quit"];

	// Some mock data
	todo_list.push(todo::TodoItem{title: "Test 1".to_string(), description: "Test description 1".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 2".to_string(), description: "Test description 2".to_string(), completed: false});
	todo_list.push(todo::TodoItem{title: "Test 3".to_string(), description: "Test description 3".to_string(), completed: false});

	// Show welcome message
	print_welcome_message(&term_size);

	// Main loop
	loop {
		print_todo_list(&todo_list, &term_size);
		helpers::print_div(helpers::DividerType::Double, &term_size);
		print_options(&options);

		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read line");
		let choice: u8 = match user_input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please enter a number");
				helpers::pause();
				continue;
			}
		};

		// Check the option is valid
		if choice < 1 || choice > options.len() as u8 {
			println!("Please enter a valid option");
			helpers::pause();
			continue;
		}

		// Run the corresponding function
		match choice {
			1 => add_item_submenu(&mut todo_list),
			2 => toggle_item_submenu(&mut todo_list),
			3 => edit_item_submenu(&mut todo_list),
			4 => delete_item_submenu(&mut todo_list),
			5 => quit(),
			_ => continue,
		}
	}
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
		println!(" {} | Title: {}, Descritpion: {}, Completed: {}", todo_list.iter().position(|x| x.title == item.title).unwrap() + 1, item.title, item.description, item.completed);
		helpers::print_div(helpers::DividerType::Single, &term_size);
	}
}

fn add_item_submenu(todo_list: &mut Vec<todo::TodoItem>) {
	let mut title = String::new();
	let mut description = String::new();

	println!("Please enter the title of the new item:");
	io::stdin().read_line(&mut title).expect("Failed to read line");
	println!("Please enter the description of the new item:");
	io::stdin().read_line(&mut description).expect("Failed to read line");

	if title.trim().is_empty() {
		println!("Title cannot be empty");
		helpers::pause();
		return;
	}

	let title = title.trim().to_string();
	let description = description.trim().to_string();

	todo::add_item(todo_list, title, description);
	println!("Item added successfully");
}

fn delete_item_submenu(todo_list: &mut Vec<todo::TodoItem>) {
	let mut user_input = String::new();
	println!("Please enter the number of the item you wish to delete:");
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let choice: u8 = match user_input.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a number");
			helpers::pause();
			return;
		}
	};

	// Check the option is valid
	if choice < 1 || choice > todo_list.len() as u8 {
		println!("Please enter a valid option");
		helpers::pause();
		return;
	}

	todo::delete_item(todo_list, choice as usize - 1);
	println!("Item deleted successfully");
}

fn toggle_item_submenu(todo_list: &mut Vec<todo::TodoItem>) {
	let mut user_input = String::new();
	println!("Please enter the number of the item you wish to toggle:");
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let choice: u8 = match user_input.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a number");
			helpers::pause();
			return;
		}
	};

	// Check the option is valid
	if choice < 1 || choice > todo_list.len() as u8 {
		println!("Please enter a valid option");
		helpers::pause();
		return;
	}

	todo::toggle_item(todo_list, choice as usize - 1);
}

fn edit_item_submenu(todo_list: &mut Vec<todo::TodoItem>) {
	let mut user_input = String::new();
	println!("Please enter the number of the item you wish to edit:");
	io::stdin().read_line(&mut user_input).expect("Failed to read line");
	let choice: u8 = match user_input.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a number");
			helpers::pause();
			return;
		}
	};

	// Check the option is valid
	if choice < 1 || choice > todo_list.len() as u8 {
		println!("Please enter a valid option");
		helpers::pause();
		return;
	}

	let mut title = String::new();
	let mut description = String::new();

	println!("Please enter a new title for the item:");
	io::stdin().read_line(&mut title).expect("Failed to read line");
	println!("Please enter a new description for the item:");
	io::stdin().read_line(&mut description).expect("Failed to read line");

	if title.trim().is_empty() {
		println!("Title cannot be empty");
		helpers::pause();
		return;
	}

	let title = title.trim().to_string();
	let description = description.trim().to_string();
	todo::edit_item(todo_list, choice as usize - 1, title, description);
}

fn quit() {
	println!("Goodbye!");
	std::process::exit(0);
}
