use crate::output::TaskContent;
use crate::result::*;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

const SAVE_FILE_NAME: &str = "task.json";

pub struct TaskHandler {
    data: TaskData,
    path: PathBuf,
}

impl TaskHandler {
    pub fn from_json(path: &Path) -> Result<Self> {
        let mut save_path = path.to_path_buf();
        save_path.push(SAVE_FILE_NAME);

        match get_save(path) {
            Some(save) => {
                let rdr = BufReader::new(save);
                let data = serde_json::from_reader(rdr)?;

                Ok(Self {
                    data,
                    path: save_path,
                })
            }
            None => {
                create_save(&save_path)?;
                Ok(Self {
                    data: TaskData::default(),
                    path: save_path,
                })
            }
        }
    }

    pub fn save(&mut self) -> Result<()> {
        let serialized = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(&self.path, serialized)?;
        Ok(())
    }

    pub fn create_task(&mut self, name: &str) -> Result<Message> {
        if !self.task_exists(name) {
            self.data.new_task(name, None);
            return Ok(Message::CreatedTask(name.to_owned()));
        }
        Err(SystemError::TaskAlreadyExists(name.to_owned()).into())
    }

    pub fn delete_task(&mut self, name: &str) -> Result<()> {
        if self.task_exists(name) {
            self.data.delete_task(name);
            return Ok(());
        }
        Err(SystemError::TaskDoesntExist(name.to_owned()).into())
    }

    pub fn edit_task(
        &mut self,
        name: &str,
        desc: Option<&str>,
        status: Option<&str>,
        new_name: Option<&str>,
    ) -> Result<Message> {
        if let Some(properties) = self.data.get_mut_task(name) {
            if let Some(description) = desc {
                properties.desc = description.to_owned();
            };
            if let Some(s) = status {
                properties.status = s.to_owned();
            }
            if let Some(new_name) = new_name {
                if self.task_exists(new_name) {
                    return Err(SystemError::TaskAlreadyExists(new_name.to_owned()).into());
                }
                let properties = self.data.delete_task(name);
                self.data.new_task(new_name, Some(properties));
            }
            Ok(Message::AppliedTaskChanges(name.to_string()))
        } else {
            Err(SystemError::TaskDoesntExist(name.to_owned()).into())
        }
    }

    pub fn task_exists(&mut self, name: &str) -> bool {
        self.data.task_exists(name)
    }

    pub fn is_empty(&self) -> bool {
        self.data.tasks.is_empty()
    }

    pub fn get_content(&self, name: &str) -> Result<TaskContent> {
        if let Some(task) = self.data.get_task(name) {
            return Ok(TaskContent::new(
                name,
                &task.desc,
                task.status.as_str().into(),
            ));
        };
        Err(SystemError::TaskDoesntExist(name.to_owned()).into())
    }

    pub fn all_content(&self) -> Vec<TaskContent> {
        let mut content = Vec::new();
        for (name, p) in &self.data.tasks {
            let task = TaskContent::new(name, &p.desc, p.status.as_str().into());
            content.push(task);
        }
        content
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
struct TaskData {
    tasks: HashMap<String, TaskProperties>,
}

impl TaskData {
    fn new_task(&mut self, name: &str, properties: Option<TaskProperties>) {
        if let Some(properties) = properties {
            self.tasks.insert(name.to_owned(), properties);
        } else {
            self.tasks.insert(name.to_owned(), TaskProperties::new());
        }
    }

    fn delete_task(&mut self, name: &str) -> TaskProperties {
        self.tasks.remove(name).unwrap()
    }

    fn get_mut_task(&mut self, name: &str) -> Option<&mut TaskProperties> {
        self.tasks.get_mut(name)
    }

    fn get_task(&self, name: &str) -> Option<&TaskProperties> {
        self.tasks.get(name)
    }

    fn task_exists(&self, name: &str) -> bool {
        self.tasks.contains_key(name)
    }
}

/// Contains all properties of a task.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct TaskProperties {
    desc: String,
    status: String,
}

impl TaskProperties {
    /// Create empty task properties.
    fn new() -> Self {
        Self {
            desc: String::from(""),
            status: String::from("a"),
        }
    }
}

fn get_save(path: &Path) -> Option<File> {
    // TODO Requires result
    for e in std::fs::read_dir(path).unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name();
            if name.to_str().unwrap().eq(SAVE_FILE_NAME) {
                return Some(std::fs::File::open(path).unwrap());
            }
        }
    }
    None
}

fn create_save(save_path: &Path) -> Result<()> {
    // TODO Requires result
    std::fs::File::create(&save_path)?;
    let default_data = TaskData::default();
    let default = serde_json::to_string(&default_data)?;
    std::fs::write(&save_path, default)?;
    Ok(())
}
