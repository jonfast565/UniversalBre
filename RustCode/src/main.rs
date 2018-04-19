
mod ordering;
mod log;
mod constants;
mod command_line;
mod repl;
mod token;
mod token_type;
mod atom_status;
mod scanner;
mod binary_expr;
mod literal_expr;
mod statement_type;
mod statement;
mod loop_type;
mod loop_block;
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
