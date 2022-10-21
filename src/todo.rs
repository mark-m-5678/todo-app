pub(crate) struct TodoItem {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub(crate) fn add_item(todo_list: &mut Vec<TodoItem>, title: String, description: String) {
    let new_item = TodoItem{title, description, completed: false};
    todo_list.push(new_item);
}

pub(crate) fn delete_item(todo_list: &mut Vec<TodoItem>, index: usize) {
    todo_list.remove(index);
}

pub(crate) fn toggle_item(todo_list: &mut Vec<TodoItem>, index: usize) {
    todo_list[index].completed = !todo_list[index].completed;
}

pub(crate) fn edit_item(todo_list: &mut Vec<TodoItem>, index: usize, title: String, description: String) {
    todo_list[index].title = title;
    todo_list[index].description = description;
}
