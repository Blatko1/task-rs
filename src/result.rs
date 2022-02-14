use std::fmt::Display;

use colored::Colorize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    err: Box<ErrorType>,
}

impl Error {
    pub fn new(err: ErrorType) -> Self {
        Self { err: Box::new(err) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl std::error::Error for Error {}

pub enum Message {
    CreatedTask(String),
    DeletedTasks(Vec<String>, Vec<String>),
    AppliedTaskChanges(String),
}

#[derive(Debug)]
pub enum ErrorType {
    System(SystemError),
    Serde(SerdeError),
    Io(std::io::Error),
}

#[derive(Debug)]
pub enum SystemError {
    TaskAlreadyExists(String),
    TaskDoesntExist(String),
    Empty,
}

#[derive(Debug)]
pub enum SerdeError {
    Serialization(serde_json::Error),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "Info".bright_green(), "~ ".bright_blue())?;
        match self {
            Message::CreatedTask(msg) => writeln!(
                f,
                "Created task {}{}{}.",
                "\"".yellow(),
                msg.yellow(),
                "\"".yellow()
            ),
            Message::DeletedTasks(msgs, errs) => {
                if !msgs.is_empty() {
                    write!(f, "Deleted: ")?;
                    let mut iter = msgs.iter().peekable();
                    loop {
                        match iter.next() {
                            Some(msg) => write!(f, "{}", msg.yellow())?,
                            None => break,
                        }
                        if iter.peek().is_some() {
                            write!(f, ", ")?;
                        }
                    }
                    writeln!(f, ".")?;
                }

                if !errs.is_empty() {
                    write!(f, "Failed to delete ")?;
                    let mut iter = errs.iter().peekable();
                    loop {
                        match iter.next() {
                            Some(msg) => write!(f, "{}", msg.yellow())?,
                            None => break,
                        }
                        if iter.peek().is_some() {
                            write!(f, ", ")?;
                        }
                    }
                    writeln!(f, ".")?;
                }
                Ok(())
            }
            Message::AppliedTaskChanges(msg) => writeln!(
                f,
                "Applied additions to {}{}{}.",
                "\"".yellow(),
                msg.yellow(),
                "\"".yellow()
            ),
        }
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::System(err) => {
                write!(f, "{}{}", "System Error".red(), "~ ".bright_blue())?;
                match err {
                    SystemError::TaskAlreadyExists(e) => writeln!(
                        f,
                        "Task {}{}{} already exists!",
                        "\"".yellow(),
                        e.yellow(),
                        "\"".yellow()
                    ),
                    SystemError::TaskDoesntExist(e) => writeln!(
                        f,
                        "Task {}{}{} doesn't exist.",
                        "\"".yellow(),
                        e.yellow(),
                        "\"".yellow()
                    ),
                    SystemError::Empty => writeln!(
                        f,
                        "The task table is empty. Create some tasks with {} command!",
                        "'new'".yellow()
                    ),
                }
            }
            ErrorType::Serde(err) => {
                write!(f, "{}{}", "Serde Error".red(), "~ ".bright_blue())?;
                match err {
                    SerdeError::Serialization(e) => writeln!(f, "{}", e),
                }
            }
            ErrorType::Io(err) => {
                writeln!(f, "{}{}{}", "Io Error".red(), "~ ".bright_blue(), err)
            }
        }
    }
}

impl From<SystemError> for Error {
    fn from(e: SystemError) -> Self {
        Error::new(ErrorType::System(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorType::Serde(SerdeError::Serialization(e)))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorType::Io(e))
    }
}
