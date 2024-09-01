use constants::BitboardRuntimeConstants;
use engine_core::engine::ab_engine::StandardAlphaBetaEngine;
use engine_core::engine::{self, Engine, SearchMetadata, ShouldAbortSearchCallback};
use move_list::MoveList;
/// Functionality for running the Universal Chess Protocol
/// 
/// This is a standardized way for chess engines to communicate.
/// See http://wbec-ridderkerk.nl/html/UCIProtocol.html for protocol
/// specification.

// Use rustyline for a better commandline experience
// Allows for line history and more
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::sync::{Arc, Mutex};
use std::{io, thread};
use std::io::BufRead;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::SystemTime;


use engine_core::core::*;
use engine_core::core::bitboard::*;
use engine_core::commands;

const ENGINE_NAME: &str = "Magnificence Oxidized";
const ENGINE_AUTHORS: &str = "William Sandstrom and Harald Bjurulf";

#[derive(Debug, PartialEq, Clone)]
struct GoState {
    // How deep is the engine allowed to search?
    depth: Option<usize>,
    // How many nodes is the engine allowed to search?
    nodes: Option<usize>,
    // White time related
    white_time: Option<usize>,
    white_time_increment: Option<usize>,
    // Black time related
    black_time: Option<usize>,
    black_time_increment: Option<usize>,
    // Restrict the search to certain moves
    search_moves: Option<Vec<String>>,
    // Restrict the search to a certain amount of time
    move_time: Option<usize>,
}

struct WorkerState {
    board_constant_state: Rc<BitboardRuntimeConstants>,
    board: Board,
    engine: Box<dyn Engine>,
    move_history: Vec<Move>,
    strict_uci_mode: bool,
}

struct SharedState {
    strict_uci_mode: bool,
    stop_search: bool,
    should_quit: bool,
    is_worker_busy: bool,
    is_worker_complete: bool
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
// UCI Command types sent from GUI to engine
enum CommandType {
    // UCI commands
    UCI,
    Debug(bool),
    IsReady,
    SetOption(String, String),
    UCINewGame,
    Position(Option<String>, Vec<String>),
    Go(GoState),
    Stop,
    Quit,
    // Non-UCI commands
    Perft(usize),
    Divide(usize),
    PerftTests,
    Move(String),
    Undo,
    DisplayBoard,
    EvaluateBoard,
    LegalMoves,
    Help,
    Unknown,
    Error(String),
}

/// Start the UCI protocol, start accepting command
pub fn start_uci_protocol(player_type: &str) {
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(".linehistory.txt");

    let (board_constant_state, duration) = timeit(|| BitboardRuntimeConstants::create());
    println!("Constant state initialization took {:.3} seconds", duration);

    println!("Type 'help' for help");

    let shared_state = Arc::new(Mutex::new(SharedState {
        strict_uci_mode: false,
        stop_search: false,
        should_quit: false,
        is_worker_busy: false,
        is_worker_complete: false
    }));
    let shared_state_worker = Arc::clone(&shared_state);
    let player_type_copy = player_type.to_string();

    let (tx, rx) : (Sender<CommandType>, Receiver<CommandType>) = mpsc::channel();
    // Spawn a worker thread performs various commands
    let worker_thread = thread::spawn(move || {
        let shared_state_worker_clone = Arc::clone(&shared_state_worker);
        let should_abort_search_callback: ShouldAbortSearchCallback = Box::new(move || {
            return shared_state_worker_clone.lock().unwrap().stop_search;
        });

        // The worker thread listens for commands
        let board_constant_state_rc = Rc::new(board_constant_state);
        let mut worker_state = WorkerState {
            board: Board::from_fen(STARTING_POS_FEN, Rc::clone(&board_constant_state_rc)),
            board_constant_state: board_constant_state_rc,
            engine: engine::from_name(&player_type_copy, Box::new(handle_search_metadata), Box::new(log_callback), should_abort_search_callback),
            move_history: Vec::new(),
            strict_uci_mode: false,
        };

        while let Ok(command) = rx.recv() {
            // Process the command
            handle_command(&command, &mut worker_state, &shared_state_worker);
            if shared_state_worker.lock().unwrap().should_quit {
                break;
            }
        }
    });

    // Read input and pass it to the worker thread.
    while !shared_state.lock().unwrap().should_quit {
        let strict_uci_mode = shared_state.lock().unwrap().strict_uci_mode;
        let line = match strict_uci_mode {
            false => read_input_uci_off(&mut rl),
            true => read_input_uci_on() 
        };
        let command = parse_command(&line);

        let mut state = shared_state.lock().unwrap();
        if tx.send(command.clone()).is_err() || command == CommandType::Quit {
            break;
        }

        if command == CommandType::Stop && !state.is_worker_complete {
            state.stop_search = true;
            drop(state);
            continue;
        }

        state.is_worker_complete = false;
        if !state.strict_uci_mode {
            drop(state);
            while !shared_state.lock().unwrap().is_worker_complete {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
        else {
            drop(state);
        }
    }

    // Wait for the worker thread to finish
    if let Err(err) = worker_thread.join() {
        println!("Worker thread encountered an error: {:?}", err);
    }
}

pub fn run_single_uci_command(command_line: &str, player_type: &str) {
    let board_constant_state = Rc::new(BitboardRuntimeConstants::create());

    let mut state = WorkerState {
        board: Board::new(Rc::clone(&board_constant_state)),
        board_constant_state,
        engine: engine::from_name(player_type, Box::new(handle_search_metadata), Box::new(log_callback), Box::new(|| false)),
        move_history: Vec::new(),
        strict_uci_mode: false,
    };

    let shared_state = Arc::new(Mutex::new(SharedState {
        strict_uci_mode: false,
        stop_search: false,
        should_quit: false,
        is_worker_busy: false,
        is_worker_complete: true
    }));

    let command = parse_command(command_line);
    handle_command(&command, &mut state, &shared_state);
}

fn search(state: &mut WorkerState, go_state: &GoState, shared_state: &Arc<Mutex<SharedState>>) {
    let pv = state.engine.search(&state.board);
    let mv = pv.first().unwrap();
    println!("bestmove {}", mv);
}

fn log_callback(text: &str) {
    println!("{}", text);
}

fn handle_command(command : &CommandType, state: &mut WorkerState, shared_state: &Arc<Mutex<SharedState>>) {
    match command {
        CommandType::Quit  => {
            shared_state.lock().unwrap().should_quit = true;
            if !state.strict_uci_mode {
                println!("Exiting...");
            }
        }
        CommandType::Error(e) if !state.strict_uci_mode => {
            println!("Error: {}", e);
        }
        CommandType::Unknown if !state.strict_uci_mode => {
            println!("Unknown command");
        },
        CommandType::IsReady if state.strict_uci_mode => {
            println!("readyok");
        },
        CommandType::UCI => {
            state.strict_uci_mode = true;
            shared_state.lock().unwrap().strict_uci_mode = true;
            uci_start();
        }
        CommandType::UCINewGame => {
            // Maybe do something here?
            
        }
        CommandType::Go(go_state) => {
            search(state, go_state, shared_state)
        },
        CommandType::Position(fen, moves) => {
            if let Some(fen_pos) = fen {
                state.board = Board::from_fen(&fen_pos, Rc::clone(&state.board_constant_state));
            }
            state.board = commands::board_from_moves(&state.board, moves);
        },
        CommandType::Perft(depth) => {
            perft(depth, state);
        },
        CommandType::Move(mv_algebraic) => {
            let mv = Move::from_algebraic(&state.board, mv_algebraic);
            state.board.make_move(&mv);
            state.move_history.push(mv);
        },
        CommandType::Undo => {
            let possible_mv = state.move_history.pop();
            if let Some(mv) = possible_mv {
                state.board.unmake_move(&mv);
                println!("Move {} was undone", mv);
            }
            else {
                println!("No moves have been made, cannot undo.");
            }
        }
        CommandType::DisplayBoard => {
            println!("{}", state.board.to_string());
        },
        CommandType::Divide(depth) => {
            divide(depth, state);
        }
        CommandType::LegalMoves => {
            let mut move_vector = MoveList::empty();
            state.board.get_moves(&mut move_vector);
            let mut moves : Vec<String> = move_vector.iter().map(|mv| mv.to_algebraic()).collect();
            moves.sort();
            println!("Legal moves ({}): {}", state.board.get_current_player().to_char(), moves.join(" "));
        }
        CommandType::PerftTests => {
            commands::perft_tests(Rc::clone(&state.board_constant_state), 20_000_000_000);
        }
        _ => {}
    };
    shared_state.lock().unwrap().is_worker_complete = true;
}

fn handle_search_metadata(metadata: SearchMetadata) {
    //println!("Go status: {:?}", metadata);
}

fn uci_start() {
    println!("id name {}", ENGINE_NAME);
    println!("id author {}", ENGINE_AUTHORS);
    println!("uciok");
}

fn perft(depth: &usize, state: &mut WorkerState) {
    println!("Performing perft of depth {}", depth);
    let mut reserved_moves : Vec<MoveList> = (0..15).map(|_| MoveList::empty()).collect();
    let (perft_count, duration) = timeit(|| commands::perft(*depth, &mut state.board, &mut reserved_moves));
    let million_moves_per_second = (perft_count / 1_000_000) as f64 / duration;
    println!("Perft completed in {:.3} seconds ({:.2}M moves per second)", duration, million_moves_per_second);
    println!("Result: {}", perft_count);
}

fn divide(depth: &usize, state: &mut WorkerState) {
    println!("Performing perft of depth {}", depth);
    let mut reserved_moves : Vec<MoveList> = (0..15).map(|_| MoveList::empty()).collect();
    let (perft_count, duration) = timeit(|| commands::divide(*depth, &mut state.board, &mut reserved_moves));
    let million_moves_per_second = (perft_count / 1_000_000) as f64 / duration;
    println!("Perft completed in {:.3} seconds ({:.2}M moves per second)", duration, million_moves_per_second);
    println!("Result: {}", perft_count);
}

// =============== Input parsing ===================

fn read_input_uci_off(rl : &mut Editor::<()>) -> String {
    // Read input using rustyline library
    let readline = rl.readline(">> ");
    let line = match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str());
            line.as_str().to_string()
        },
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            return "quit".to_string();
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return "quit".to_string();
        }
    };
    return line;
}

fn read_input_uci_on() -> String {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    return line;
}

fn parse_command(line: &str) -> CommandType {
    // Parse input into a command
    let split = line.trim().split(" ");
    let words = split.collect::<Vec<&str>>();

    let command = match words[0] {
        "quit" | "exit" | "q" | "quit()" | "exit()" => CommandType::Quit,
        "stop" => CommandType::Stop,
        "uci" => CommandType::UCI,
        "ucinewgame" => CommandType::UCINewGame,
        "isready" => CommandType::IsReady,
        "help" | "h" => CommandType::Help,
        "display" | "disp" | "d" | "board" | "show" => CommandType::DisplayBoard,
        "eval" | "evaluate" | "score" => CommandType::EvaluateBoard,
        "divide" | "div" => {
            if words.len() > 1 {
                match words[1].parse::<usize>() {
                    Ok(n) => CommandType::Divide(n),
                    Err(_) => CommandType::Error("Invalid divide perft depth".to_string())
                }
            }
            else {
                CommandType::Error("Please specify a divide perft depth".to_string())
            }
        }
        "move" | "makemove" | "mv" | "make" => {
            if words.len() > 1 {
                CommandType::Move(words[1].to_string())
            }
            else {
                CommandType::Error("Please a move in algebraic form".to_string())
            }
        }
        "position" | "pos" | "setboard" | "p" => { 
            parse_uci_position_cmd(&words[1..])
        }
        "go" => { 
            parse_uci_command_go(&words[1..]) 
        }
        "perft" => {
            if words.len() > 1 {
                match words[1].parse::<usize>() {
                    Ok(n) => CommandType::Perft(n),
                    Err(_) => CommandType::Error("Invalid perft depth".to_string())
                }
            }
            else {
                CommandType::Error("Please specify a perft depth".to_string())
            }
        }
        "moves" | "getmoves" | "legalmoves" | "mvs" => CommandType::LegalMoves,
        "undo" | "unmake" => CommandType::Undo,
        "perfttests" | "perftest" | "testperft" => CommandType::PerftTests,
        _ => CommandType::Unknown
    };
    return command;
}

// Get a named argument value from a list of words, as an integer.
// Returns None if no arguments found
fn get_named_argument_as_num(words : &[&str], name: &str) -> Option<usize> {
    match get_named_argument(words, name) {
        Some(n) => {
            match n.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            }
        }
        None => None
    }
}

// Get a named arguments value from a list of words
// Returns None if no arguments found
fn get_named_argument(words : &[&str], name: &str) -> Option<String> {
    for (i, val) in words.iter().enumerate() {
        if *val == name {
            return if i < (words.len() - 1) {
                Some(words[i + 1].to_string())
            }
            else {
                Some("".to_string())
            };
        }
    }
    return None;
}

// Parse the UCI 'position' command into a CommandType::Position(FEN, MOVES)
// Returns a CommandType::Error if the command is not well formed
fn parse_uci_position_cmd(words_slice : &[&str]) -> CommandType {
    let words = Vec::from(words_slice);
    if words.len() > 0 {
        let moves_cmd_index = words.iter().position(|x| *x == "moves" || *x == "mv");

        // Isolate first part: fen/startpos
        let fen_command = match words[0] {
            "fen" if moves_cmd_index.is_some() => Some(Vec::from(&words[0..moves_cmd_index.unwrap()])),
            "startpos" | "sp" => Some(vec!["startpos"]),
            _ if !words.contains(&"moves") => Some(words.clone()),
            "moves" | "mv" => None,
            _ => None
        };
        
        // Isolate last part: moves
        let move_command = match words[0] {
            "fen" | "startpos" if moves_cmd_index.is_some() => Some(Vec::from(&words[moves_cmd_index.unwrap()..])),
            "moves" | "mv" => Some(words.clone()),
            _ => None,
        };
        
        let fen = match fen_command {
            Some(command) => {
                match command[0] {
                    "fen" => Some(command[1..].join(" ").to_string()),
                    "startpos" => Some(STARTING_POS_FEN.to_string()),
                    _ => Some(words[0..].join(" ").to_string()) 
                }
            }
            None => None
        };

        let move_list = match move_command {
            Some(command) => command[1..].iter().map(|word| word.to_string()).collect::<Vec<String>>().to_vec(),
            None => Vec::new(),
        };

        if move_list.len() == 0 && fen.is_none() {
            return CommandType::Error("Please specify position arguments".to_string());
        }

        return CommandType::Position(fen, move_list);
    } else {
        return CommandType::Error("Please specify position arguments".to_string());
    }
}

// Parse the UCI 'go' command into a CommandType::Go
// Returns a CommandType::Error if the command is not well formed
fn parse_uci_command_go(words : &[&str]) -> CommandType {
    let mut go_state : GoState = GoState {
        depth: None,
        nodes: None,
        white_time: None,
        white_time_increment: None,
        black_time: None,
        black_time_increment: None,
        move_time: None,
        search_moves: None
    };

    go_state.depth = get_named_argument_as_num(&words, "depth");
    go_state.nodes = get_named_argument_as_num(&words, "nodes");
    go_state.white_time = get_named_argument_as_num(&words, "wtime");
    go_state.white_time_increment = get_named_argument_as_num(&words, "winc");
    go_state.black_time = get_named_argument_as_num(&words, "btime");
    go_state.black_time_increment = get_named_argument_as_num(&words, "binc");
    go_state.move_time = get_named_argument_as_num(&words, "movetime");

    // Use an infinite depth if infinite is stated
    if get_named_argument(&words, "infinite") != None || words.len() == 0 {
        go_state.depth = Some(1000);
    }
    // If only one argument is specified, treat it as depth
    else if words.len() == 1 {
        go_state.depth = match words[0].parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    return CommandType::Go(go_state);
}

fn timeit<F: FnMut() -> T, T>(mut f: F) -> (T, f64) {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    return (result, duration.as_secs_f64());
}
  