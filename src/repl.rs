extern crate rustyline;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use log;
use scanner;
use parser;

use token::{Token, TokenType};

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
    log::log_info("REPL: Enter + CTRL-D to run commands, or CTRL-C to exit");

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

            let mut scanner = scanner::Scanner::init(repl_status.input);
            let tokens = scanner.scan_all();
            match tokens {
                Ok(token_list) => { 
                    print_tokens(token_list.to_vec());
                    let mut parser = parser::Parser::init(token_list/*.to_vec()*/);
                    match parser.parse() {
                        Ok(blocks) => {
                            // TODO: implement code generation
                        },
                        Err(contents) => {
                            log::log_error(&format!("CE {}", contents.to_string()));
                        }
                    }
                },
                Err(scan_error) => println!("Failed! {}", scan_error.get_error_message())
            }      
        }
    }
}
