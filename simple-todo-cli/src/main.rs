use std::{
    fmt,
    io::{self, Write},
};

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

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The todo title is {} and desc is {}",
            self.title,
            self.desc.as_deref().unwrap_or("empty")
        )
    }
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

enum YesOrNo {
    YES,
    NO,
}

fn say_yes_or_no(message: &str, default_value: YesOrNo, cb: &mut dyn FnMut(YesOrNo) -> ()) {
    let default_tip = match default_value {
        YesOrNo::YES => "Y/n",
        YesOrNo::NO => "y/N",
    };
    let mut input = String::new();
    print!("{} ({}): ", message, default_tip);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().as_ref() {
        "y" | "Y" => cb(YesOrNo::YES),
        "n" | "N" => cb(YesOrNo::NO),
        "" => cb(default_value),
        _ => {
            println!("error input");
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

    say_yes_or_no(
        "Should print todo data?",
        YesOrNo::YES,
        &mut |flag| match flag {
            YesOrNo::YES => {
                todo_data.iter_mut().enumerate().for_each(|(index, item)| {
                    item.toggle_status();
                    item.toggle_status();
                    println!("[{}] {}", index, item,);
                });
            }
            YesOrNo::NO => {
                println!("end");
            }
        },
    );
}
