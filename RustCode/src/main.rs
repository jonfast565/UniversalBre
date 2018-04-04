mod log;
mod scanner;
mod parser;
mod constants;
mod command_line;
mod repl;

fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
