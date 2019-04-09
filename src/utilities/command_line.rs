extern crate clap;

use self::clap::{App /*, SubCommand */, Arg};
use std::env;

pub struct CommandLineArguments {
    pub interactive : bool,
    pub file_path : String,
}

pub fn get_arguments() -> CommandLineArguments {
    let _matches = App::new("Universal BRE")
        .version("0.1")
        .author("Jon Fast	<jnfstdj656@gmail.com>")
        .about("Compiles and interprets BRE rules")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .value_name("INTERACTIVE")
                .help("Determines whether the user has a REPL to enter commands")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file_path")
                .short("fp")
                .long("file_path")
                .value_name("FILE PATH")
                .help("A file path to a program that should be compiled")
                .takes_value(true),
        )
        .get_matches();

    let current_path = env::current_dir().unwrap();
    let test_program = format!("{}{}", current_path.to_str().unwrap(), "\\test_programs\\feature_test.prg");

    CommandLineArguments {
        interactive: _matches.value_of("interactive").unwrap_or("false") == "true",
        file_path: _matches.value_of("file_path").unwrap_or(&test_program).to_string(),
    }
}
