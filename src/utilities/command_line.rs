extern crate clap;
use self::clap::{App, Arg};

pub struct CommandLineArguments {
    pub interactive : bool,
    pub file_path : String,
}

pub fn get_arguments() -> CommandLineArguments {
    let matches = App::new("Universal BRE")
        .version("0.1")
        .author("Jon Fast	<jnfstdj656@gmail.com>")
        .about("Compiles and interprets BRE rules")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Determines whether the user has a REPL to enter commands")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("file_path")
                .short("fp")
                .long("file path")
                .value_name("FILE PATH")
                .help("A file path to a program that should be compiled")
                .takes_value(true),
        )
        .get_matches();

    CommandLineArguments {
        interactive: matches.is_present("interactive"),
        file_path: matches.value_of("file_path").unwrap_or("./test_programs/minimal_test.prg").to_string(),
    }
}
