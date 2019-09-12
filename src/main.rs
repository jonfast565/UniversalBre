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

use std::thread;

const STACK_SIZE: usize = 4 * 1024 * 1024;

fn run() {
    log::log_debug(constants::PROGRAM_HEADER);
    let args = command_line::get_arguments();
    if args.interactive {
        log::log_success("Initialized");
        repl::prompt_loop();
        log::log_success("Goodbye!");
    } else {
        let file_path = args.file_path;
        dbg!(file_path.to_string());
        let contents = utility::read_file(&file_path).unwrap();
        repl::compile(contents);
    }
} 

fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .name("main".into())
        .spawn(run)
        .unwrap();
    child.join().unwrap();
}
