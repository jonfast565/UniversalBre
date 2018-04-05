extern crate rustyline;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;
use log;
use scanner;

struct ReplStatus {
    input: String,
    interrupted: bool
}

impl ReplStatus {
    fn init(input: String, interrupted: bool) -> ReplStatus {
        ReplStatus {
            input: input,
            interrupted: interrupted
        }
    }
}

fn prompt() -> ReplStatus {
    log::log_info("REPL: User CTRL-D to complete input, or CTRL-C to exit");

    let mut rl = Editor::<()>::new();
    let mut result = String::new();
    let mut interrupted = false;

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                result += &line;
                result += "\n";
            }
            Err(ReadlineError::Interrupted) => {
                interrupted = true;
                break;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                log::log_error(&format!("Error: {:?}", err));
                break;
            }
        }
    }

    if !interrupted {
        return ReplStatus::init(result, false)
    }

    ReplStatus::init(String::new(), true)
}

pub fn prompt_loop() {
    let mut result = String::new();
    loop {
        let repl_status = prompt();
        if (repl_status.interrupted) {
            break
        } else {
            log::log_success(&format!("Scanning {}", repl_status.input));
            if (repl_status.input.is_empty()) { 
                log::log_info("Input empty... try again.");
                continue 
            }

            let mut scanner1 = scanner::Scanner::init(repl_status.input);
            let tokens = scanner1.scan_all();
            // debug token printer
            match tokens {
                Ok(token_list) => {
                    for token in &token_list {
                        println!("{:?}", token.get_token_type());
                    }
                },
                Err(scan_error) => println!("Failed!")
            }      
        }
    }
}
