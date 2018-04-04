mod log;
mod scanner;
mod parser;
mod constants;
mod commandline;
mod repl;

fn main() {
    commandline::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
