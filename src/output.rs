use std::{
    fmt::Display,
    io::{self, Write},
};

use colored::Colorize;
use comfy_table::{presets, Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

const TABLE_LEGEND: &str = "\n✅ - completed, 🟢 - active, 🟡 - paused, 🔴 - canceled";

#[derive(Debug)]
pub struct TaskContent {
    pub name: String,
    pub desc: String,
    pub status: Status,
}

impl TaskContent {
    pub fn new(name: &str, desc: &str, status: Status) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            status,
        }
    }

    pub fn sort_by(vec: &mut Vec<Self>, order: SortOrder) {
        match order {
            SortOrder::Alphabetical => vec.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOrder::ReverseAlphabetical => vec.sort_by(|a, b| b.name.cmp(&a.name)),
            SortOrder::Status => vec.sort_by(|a, b| a.status.cmp(&b.status)),
            SortOrder::ReverseStatus => vec.sort_by(|a, b| b.status.cmp(&a.status)),
        }
    }
}

#[derive(Debug, Eq, Clone, Copy)]
pub enum Status {
    Active,
    Stopped,
    Canceled,
    Completed,
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Completed => write!(f, "✅"),
            Status::Active => write!(f, "🟢"),
            Status::Stopped => write!(f, "🟡"),
            Status::Canceled => write!(f, "🔴"),
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "f" => Self::Completed,
            "a" => Self::Active,
            "s" => Self::Stopped,
            "c" => Self::Canceled,
            &_ => unreachable!("Unreachable"),
        }
    }
}

pub struct Output {
    stdout: io::Stdout,
}

impl Output {
    pub fn init() -> Self {
        let stdout = io::stdout();

        Self { stdout }
    }

    pub fn write<T: std::fmt::Display>(&mut self, msg: T) {
        let mut f = self.stdout.lock();
        write!(f, "{}", msg).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::abort();
        })
    }

    pub fn write_all<T: std::fmt::Display>(&mut self, msgs: Vec<T>) {
        for msg in msgs {
            self.write(msg);
        }
    }

    pub fn fatal_error<T: std::fmt::Display>(&mut self, msg: T) -> ! {
        self.write("FATAL ERROR:".on_red());
        self.write(" ");
        self.write(msg);
        std::process::abort();
    }

    pub fn print_table(&mut self, mut content: Vec<TaskContent>, order: SortOrder) {
        TaskContent::sort_by(&mut content, order);
        let mut table = Table::new();
        table
            .set_table_width(67)
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Name").add_attribute(Attribute::Bold),
                Cell::new("Status").add_attribute(Attribute::Bold),
                Cell::new("Description").add_attribute(Attribute::Bold),
            ]);

        for task in &content {
            table.add_row(vec![
                Cell::new(&task.name).fg(Color::Yellow),
                Cell::new(task.status).set_alignment(CellAlignment::Center),
                Cell::new(&task.desc),
            ]);
        }

        self.write(table);
        self.write(TABLE_LEGEND);
    }

    pub fn print_task(&mut self, task: TaskContent) {
        let mut table = Table::new();
        table
            .set_table_width(40)
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Name", "Status", "Description"])
            .add_row(vec![
                Cell::new(task.name),
                Cell::new(task.status).set_alignment(CellAlignment::Center),
                Cell::new(task.desc),
            ]);

        self.write(table);
        self.write(TABLE_LEGEND);
    }
}

//TODO
pub enum SortOrder {
    Alphabetical,
    ReverseAlphabetical,
    Status,
    ReverseStatus,
}
