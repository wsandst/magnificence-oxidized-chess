/// Functionality for running the Universal Chess Protocol
/// 
/// This is a standardized way for chess engines to communicate

// UCI Command types sent from GUI to engine
enum CommandType {
    UCI,
    Debug(bool),
    IsReady,
    SetOption(String, String),
    UCINewGame,
    PositionFen(String),
    PositionMoves(String),
    Go,
    Stop,
    Quit,
    Error(String),
}

/// Start the UCI protocol, start accepting command
pub fn start_uci_protocol() {
    let mut exit = false;

    while !exit {
        exit = process_uci_command();
    }
}

fn receive_command() {
    
}

fn uci_start() {

}

fn process_uci_command() -> bool {

    return false;
}