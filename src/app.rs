use clap::{App, Arg, PossibleValue};

pub fn get_app() -> App<'static> {
    App::new("task")
        .author("Leon")
        .version("1.0")
        .about("App tracking tasks")
        .arg(
            Arg::new("table")
                .help(
                    "Prints sorted table of tasks, sorting: alphabetical, reverse alphabetical, status, reverse status",
                )
                .long("table")
                .short('t')
                .takes_value(true)
                .value_name("sort_by")
                .possible_value("a")
                .possible_value("ra")
                .possible_value("s")
                .possible_value("rs")
                .default_missing_value("a")
        )
        .arg(
            Arg::new("info")
                .help("Prints information about a task")
                .long("info")
                .alias("print")
                .short('i')
                .takes_value(true)
                .value_name("name"),
        )
        .arg(
            Arg::new("delete")
                .help("Deletes a task")
                .long("delete")
                .alias("del")
                .alias("remove")
                .short('d')
                .takes_value(true)
                .value_name("name")
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("complete")
                .help("Sets task's status to completed")
                .long("complete")
                .alias("done")
                .alias("finish")
                .short('f')
                .takes_value(true)
                .value_name("name"),
        )
        .arg(
            Arg::new("active")
                .help("Sets task's status to active")
                .long("active")
                .short('a')
                .takes_value(true)
                .value_name("name"),
        )
        .arg(
            Arg::new("stop")
                .help("Sets task's status to completed")
                .long("stop")
                .alias("pause")
                .alias("halt")
                .short('s')
                .takes_value(true)
                .value_name("name"),
        )
        .arg(
            Arg::new("cancel")
                .help("Sets task's status to canceled")
                .long("cancel")
                .short('c')
                .takes_value(true)
                .value_name("name"),
        )
        .subcommand(
            App::new("new")
                .about("Creates a new task")
                .alias("add")
                .arg(
                    Arg::new("name")
                        .help("Name of a new task, tasks can't have same names")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .help("Sets description for this task")
                        .alias("desc")
                        .short('d')
                        .takes_value(true)
                        .value_name("desc"),
                )
                .arg(status_arg()),
        )
        .subcommand(
            App::new("edit")
                .about("Edits properties of a task")
                .arg(
                    Arg::new("task")
                        .help("Name of a task you want to edit")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .help("Sets description for this task")
                        .takes_value(true)
                        .alias("desc")
                        .value_name("desc")
                        .short('d'),
                )
                .arg(
                    Arg::new("rename")
                        .help("Renames this task")
                        .long("rename")
                        .short('r')
                        .takes_value(true)
                        .value_name("new_name"),
                )
                .arg(status_arg()),
        )
}

fn status_arg() -> Arg<'static> {
    Arg::new("status")
        .help("Sets task's status, statuses: completed, active, stopped, canceled")
        .long("status")
        .short('s')
        .takes_value(true)
        .value_name("status")
        .possible_value(PossibleValue::new("f").help("Task is completed"))
        .possible_value(PossibleValue::new("a").help("Task is currently in progress"))
        .possible_value(PossibleValue::new("s").help("Task is currently not being worked on"))
        .possible_value(PossibleValue::new("n").help("Task is canceled"))
}
