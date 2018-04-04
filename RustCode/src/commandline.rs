extern crate clap;

use self::clap::{Arg, App /*, SubCommand */};

pub fn get_arguments() {
    let _matches = App::new("Universal BRE")
    .version("0.1")
    .author("Jon Fast	<jnfstdj656@gmail.com>")
    .about("Compiles and interprets BRE rules")
    .arg(Arg::with_name("interactive")
         .short("i")
         .long("interactive")
         .value_name("INTERACTIVE")
         .help("Determines whether the user has a REPL to enter commands")
         .takes_value(true))
    .get_matches();

    // let interactive_config = matches.value_of("interactive").unwrap_or("true");
    // println!("Value for config: {}", interactive_config);
}