
mod log;
mod error;
mod constants;
mod command_line;
mod repl;
mod token;
mod atom_status;
mod scanner;
mod semantic_blocks;
mod parser;

fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
