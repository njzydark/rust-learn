use std::io::{self, Write};

#[derive(Debug, PartialEq)]
enum TodoItemStatus {
    DOING,
    FINISH,
}

#[derive(Debug)]
struct TodoItem {
    title: String,
    desc: Option<String>,
    status: TodoItemStatus,
}

impl TodoItem {
    fn toggle_status(&mut self) {
        if self.status == TodoItemStatus::DOING {
            self.status = TodoItemStatus::FINISH;
        } else {
            self.status = TodoItemStatus::DOING;
        }
    }
}

fn add_todo(todo_data: &mut Vec<TodoItem>) {
    let mut todo_title_input = String::new();
    print!("Please input todo title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut todo_title_input).unwrap();

    let mut todo_desc_input = String::new();
    print!("Please input todo desc: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut todo_desc_input).unwrap();

    todo_data.push(TodoItem {
        title: todo_title_input.trim().to_string(),
        desc: Some(todo_desc_input.trim().to_string()),
        status: TodoItemStatus::DOING,
    });
}

fn main() {
    let mut todo_data: Vec<TodoItem> = vec![];

    add_todo(&mut todo_data);
    add_todo(&mut todo_data);

    todo_data.iter_mut().enumerate().for_each(|(index, item)| {
        item.toggle_status();
        item.toggle_status();
        println!("[{}] {:?}", index, item,);
    });
}
