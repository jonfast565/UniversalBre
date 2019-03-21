mod utilities;
mod scanner;
mod parser;
mod code_generation;
mod semantic_analysis;

use utilities::command_line;
use utilities::log;
use utilities::repl;
use utilities::constants;

fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
