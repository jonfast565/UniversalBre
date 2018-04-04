mod log;
mod scanner;
mod parser;

fn main() {
    log::log_info("=== Universal BRE ===");
    log::log_debug("Program warmup.");
    log::log_error("This is an error!!!!");
}
