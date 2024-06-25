mod core;
mod uci;
mod engine;
use clap::Arg;

fn main() {
    // Use clap to parse command line arguments
    let matches = clap::App::new("Magnificence Oxidized Chess Engine")
    .version("0.1")
    .author("William Sandström and Harald Bjurulf")
    .about("Magnificence Oxidized is a Chess Engine written in Rust and version three of the Magnificence line of Chess Engines.")
    .arg(Arg::new("command")
        .help("Select a UCI command to run at engine start.")
        .short('c')
        .long("command")
        .value_name("COMMAND")
        .takes_value(true)
        .max_values(10))
    .get_matches();

    if let Some(values) = matches.values_of("command") {
        // Run a single UCI command
        let cmd = values.collect::<Vec<&str>>().join(" ");
        uci::run_single_uci_command(&cmd)
    } else {
        // Start UCI protocol
        uci::start_uci_protocol();
    }
}
