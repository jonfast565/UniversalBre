extern crate rustyline;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;
use log;

fn prompt() -> String {
	log::log_info("REPL: User CTRL-D to complete input, or CTRL-C to exit");

	let mut rl = Editor::<()>::new();
	let mut result = String::new();
	let mut interrupted = false;

	loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                result += &line;
            },
            Err(ReadlineError::Interrupted) => {
                interrupted = true;
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                log::log_error(&format!("Error: {:?}", err));
                break
            }
        }
    }

    if !interrupted {
    	return result
	}

	String::from("<<INTERRUPTED>>")
}

pub fn prompt_loop() {
	loop {
		let input = prompt();
		if input == "<<INTERRUPTED>>" {
			break;
		}
	}
}