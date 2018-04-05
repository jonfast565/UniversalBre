mod log;
mod constants;
mod command_line;
mod repl;
mod token;
mod token_type;
mod atom_status;
mod scanner;
mod operation_type;
mod parser;


fn main() {
    command_line::get_arguments();
    log::log_debug(constants::PROGRAM_HEADER);
    log::log_success("Initialized");
    let input = repl::prompt_loop();
    log::log_success(&format!("Scanning {}", input));
    let mut scanner1 = scanner::Scanner::init(input);
    let tokens = scanner1.scan_all();
    match tokens {
    	Ok(token_list) => {
    		for token in &token_list {
    			println!("{:?}", token.get_token_type());
    		}
    	},
    	Err(scan_error) => println!("Failed!")
    }

    log::log_success("Goodbye!");
}
