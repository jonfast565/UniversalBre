extern crate rustyline;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use utilities::log;
use parser::parser;
use scanner::scanner;
use utilities::utility;

use scanner::token::{Token, TokenType};
use code_generation::visualizer::Visualizer;
use code_generation::mir::MirInstructionGenerator;
use code_generation::codegen::FasmGenerator;

struct ReplStatus {
    input: String,
    interrupted: bool,
}

impl ReplStatus {
    fn init(input: String, interrupted: bool) -> ReplStatus {
        ReplStatus {
            input: input,
            interrupted: interrupted,
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
        return ReplStatus::init(result, false);
    }

    ReplStatus::init(String::new(), true)
}

fn print_tokens(token_list: Vec<Token>) {
    for token in &token_list {
        let tt = token.get_token_type();
        if tt == TokenType::Identifier
            || tt == TokenType::IntegerLiteral
            || tt == TokenType::FloatLiteral
            || tt == TokenType::StringLiteral
        {
            let token_string = format!("{:?}: {}", token.get_token_type(), token.get_lexeme());
            dbg!(token_string);
        } else {
            let token_type_string = format!("{:?}", token.get_token_type());
            dbg!(token_type_string);
        }
    }
}

pub fn prompt_loop() {
    loop {
        let repl_status = prompt();
        if repl_status.interrupted {
            break;
        } else {
            if repl_status.input.is_empty() {
                log::log_info("Input empty... try again.");
                continue;
            } else {
                compile(repl_status.input);
            }
        }
    }
}

pub fn compile(input: String) -> () {
    log::log_success(&format!("Scanning {}", input));
    let mut scanner = scanner::Scanner::init(input);
    let tokens = scanner.scan_all();
    match tokens {
        Ok(token_list) => {
            print_tokens(token_list.to_vec());
            let mut parser = parser::Parser::init(token_list);
            let program = parser.parse();
            match program {
                Ok(program) => {
                    // graphviz output
                    let _ = utility::write_file(&"gviz.txt".to_string(), &program.build_graphviz());
                    // mir generator
                    dbg!(&program);
                    let mir_instructions = MirInstructionGenerator{ debug: true }.generate_mir(&program);
                    // assembly generator
                    let asm_instructions = FasmGenerator{ debug: true }.generate_asm(&mir_instructions);
                    let _ = utility::write_file(&"asm.txt".to_string(), &asm_instructions);
                    log::log_info("Success!");
                }
                Err(contents) => {
                    log::log_error(&format!("CE {}", contents.to_string()));
                }
            }
        }
        Err(scan_error) => println!("CE {}", scan_error.get_error_message()),
    }
}
