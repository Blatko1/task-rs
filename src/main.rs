use app::get_app;

mod app;
mod commands;
mod handler;
mod output;
mod result;

fn main() {
    let matches = get_app().get_matches();
    let mut output = output::Output::init();

    let mut current_path = std::env::current_exe().unwrap();
    current_path.pop();
    let mut handler = match handler::TaskHandler::from_json(&current_path) {
        Ok(h) => h,
        Err(e) => output.fatal_error(e),
    };

    commands::process_matches(&matches, &mut handler, &mut output);

    match handler.save() {
        Ok(_) => (),
        Err(e) => output.fatal_error(e),
    };
}
