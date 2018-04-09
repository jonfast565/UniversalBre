extern crate rustyline;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use log;
use scanner;
use token_type::TokenType;
use token::Token;

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

fn print_tokens(token_list: Vec<Token>) {
    for token in &token_list {
        let tt = token.get_token_type().clone();
        if tt == TokenType::Identifier
            || tt == TokenType::IntegerLiteral
            || tt == TokenType::FloatLiteral
            || tt == TokenType::StringLiteral {
            println!("{:?}: {}", token.get_token_type(), token.get_lexeme());
        } else {
            println!("{:?}", token.get_token_type());
        }
    }
}

pub fn prompt_loop() {
    let mut result = String::new();
    loop {
        let repl_status = prompt();
        if repl_status.interrupted {
            break
        } else {
            log::log_success(&format!("Scanning {}", repl_status.input));
            if repl_status.input.is_empty() { 
                log::log_info("Input empty... try again.");
                continue 
            }

            let mut scanner1 = scanner::Scanner::init(repl_status.input);
            let tokens = scanner1.scan_all();
            // debug token printer
            match tokens {
                Ok(token_list) => print_tokens(token_list),
                Err(scan_error) => println!("Failed! {}", scan_error.get_error_message())
            }      
        }
    }
}
