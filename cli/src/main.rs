mod uci;
use clap::Arg;

const USE_FANCY_SPLASH: bool = true;
const FANCY_SPLASH: &str = 
"-------------------------------------------------------------
   
                     .       |         .    .
               .  *         -*-          *
                    \\        |         /   .
   .    .            .      /^\\     .              .    .
      *    |\\   /\\    /\\  / / \\ \\  /\\    /\\   /|    *
    .   .  |  \\ \\/ /\\ \\ / /     \\ \\ / /\\ \\/ /  | .     .
            \\ | _ _\\/_ _ \\_\\_ _ /_/_ _\\/_ _ \\_/
              \\  *  *  *   \\ \\/ /  *  *  *  /
               ` ~ ~ ~ ~ ~  ~\\/~ ~ ~ ~ ~ ~ '
  
                    Magnificence Oxidized
                       By the Prog Boys
  
-------------------------------------------------------------";
const REGULAR_SPLASH: &str = 
"----------------------------------
Magnificence Oxidized Chess Engine
    Created by the Prog Boys
----------------------------------";

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
        .max_values(30))
    .arg(Arg::new("player")
        .help("Choose the engine player type.")
        .short('p')
        .long("player")
        .value_name("PLAYER")
        .possible_values(vec!["magnificence", "random"]) 
        .default_value("magnificence")   
        .takes_value(true))   
    .arg(Arg::new("nosplash")
        .help("Prevent the cli from showing a fancy splash at start.")
        .short('q')
        .long("nosplash")
        .value_name("NOSPLASH")
        .takes_value(false))
    .get_matches();

    let player = matches.value_of("player").unwrap();

    if let Some(values) = matches.values_of("command") {
        // Run a single UCI command
        let string = values.collect::<Vec<&str>>().join(" ");
        let cmds: Vec<&str> = string.split(" and ").collect();
        for cmd in cmds {
            uci::run_single_uci_command(&cmd.trim(), player)
        }
    } else {
        // Show splash screen
        if USE_FANCY_SPLASH && !matches.values_of("nosplash").is_some() {
            println!("{}\n", FANCY_SPLASH);
        }
        else {
            println!("{}\n", REGULAR_SPLASH);
        }

        // Start UCI protocol
        uci::start_uci_protocol(player);
    }
}
