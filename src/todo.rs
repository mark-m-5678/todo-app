pub(crate) struct TodoItem {
	pub title: String,
	pub description: String,
	pub completed: bool,
}

pub(crate) fn add_item(todo_list: &mut Vec<TodoItem>, title: String, description: String) {
	let new_item = TodoItem{title, description, completed: false};
	todo_list.push(new_item);
}