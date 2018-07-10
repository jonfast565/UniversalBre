mod atom_status;
mod command_line;
mod constants;
mod error;
mod log;
mod parser;
mod repl;
mod scanner;
mod semantic_blocks;
mod token;
mod utility;
mod visualizer;

fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
