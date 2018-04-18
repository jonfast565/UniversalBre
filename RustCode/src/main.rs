
mod log;
mod constants;
mod command_line;
mod repl;
mod token;
mod token_type;
mod atom_status;
mod scanner;
mod statement;
mod operation_type;
mod parser;
mod program;

fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    repl::prompt_loop();
    log::log_success("Goodbye!");
}
