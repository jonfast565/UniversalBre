mod utilities;
mod scanner;
mod parser;
mod code_generation;
mod semantic_analysis;

use utilities::command_line;
use utilities::log;
use utilities::repl;
use utilities::constants;
use utilities::utility;

fn main() {
    log::log_debug(constants::PROGRAM_HEADER);
    let args = command_line::get_arguments();
    if args.interactive {
        log::log_success("Initialized");
        repl::prompt_loop();
        log::log_success("Goodbye!");
    } else {
        let file_path = args.file_path;
        let contents = utility::read_file(&file_path).unwrap();
        repl::compile(contents);
    }
}
