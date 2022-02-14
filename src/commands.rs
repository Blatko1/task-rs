use clap::ArgMatches;

use crate::{
    handler::TaskHandler,
    output::{Output, SortOrder},
    result::*,
};

pub fn process_matches(matches: &ArgMatches, handler: &mut TaskHandler, output: &mut Output) {
    let mut messages: Vec<Message> = Vec::new();
    let mut errors: Vec<Error> = Vec::new();
    if let Some(sort) = matches.value_of("table") {
        table_cmd(handler, output, sort).unwrap_or_else(|e| errors.push(e));
    }

    if let Some(name) = matches.value_of("info") {
        task_info_cmd(handler, output, name).unwrap_or_else(|e| errors.push(e));
    }

    if let Some(values) = matches.values_of("delete") {
        let names = values.collect();
        match delete_cmd(handler, names) {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        };
    }

    if let Some(name) = matches.value_of("complete") {
        match status_cmd(handler, name, "completed") {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        }
    }

    if let Some(name) = matches.value_of("active") {
        match status_cmd(handler, name, "active") {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        }
    }

    if let Some(name) = matches.value_of("stop") {
        match status_cmd(handler, name, "stopped") {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        }
    }

    if let Some(name) = matches.value_of("cancel") {
        match status_cmd(handler, name, "canceled") {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        }
    }

    if let Some((name, args)) = matches.subcommand() {
        let result = match name {
            "new" => new_cmd(args, handler),
            "edit" => edit_cmd(args, handler),
            _ => unreachable!("Unreachable!"),
        };
        match result {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push(e),
        }
    }

    output.write_all(messages);
    output.write_all(errors);
}

fn table_cmd(handler: &TaskHandler, output: &mut Output, sort: &str) -> Result<()> {
    if !handler.is_empty() {
        let content = handler.all_content();
        match sort {
                "a" => output.print_table(content, SortOrder::Alphabetical),
                "ra" => output.print_table(content, SortOrder::ReverseAlphabetical),
                "s" => output.print_table(content, SortOrder::Status),
                "rs" => output.print_table(content, SortOrder::ReverseStatus),
                &_ => unreachable!("Unreachable!"),
        }
        return Ok(());
    }
    Err(SystemError::Empty.into())
}

fn task_info_cmd(handler: &TaskHandler, output: &mut Output, name: &str) -> Result<()> {
    let task = handler.get_content(name)?;
    output.print_task(task);
    Ok(())
}

fn new_cmd(args: &ArgMatches, handler: &mut TaskHandler) -> Result<Message> {
    let name = args.value_of("name").unwrap();
    let msg = handler.create_task(name)?;
    let desc = args.value_of("description");
    let status = args.value_of("status");
    handler.edit_task(name, desc, status, None)?;
    Ok(msg)
}

fn status_cmd(handler: &mut TaskHandler, name: &str, status: &str) -> Result<Message> {
    handler.edit_task(name, None, Some(status), None)
}

fn edit_cmd(args: &ArgMatches, handler: &mut TaskHandler) -> Result<Message> {
    let name = args.value_of("task").unwrap();
    let desc = args.value_of("description");
    let status = args.value_of("status");
    let new_name = args.value_of("rename");
    handler.edit_task(name, desc, status, new_name)
}

fn delete_cmd(handler: &mut TaskHandler, names: Vec<&str>) -> Result<Message> {
    let mut deleted = Vec::new();
    let mut errs = Vec::new();
    for name in names {
        match handler.delete_task(name) {
            Ok(_) => deleted.push(name.to_string()),
            Err(_) => errs.push(name.to_string()),
        };
    }
    Ok(Message::DeletedTasks(deleted, errs))
}
