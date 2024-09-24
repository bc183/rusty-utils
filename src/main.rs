mod commands;

use std::env;

use commands::format::FormatCommand;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Simple argument parsing
    if args.len() != 3 {
        eprintln!("Usage: rusty_utils format <file_path>");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    // Handle the 'format' command for JSON files
    if command == "format" {
        let format_command = FormatCommand::new(file_path.to_string());
        match format_command.format() {
            Ok(_) => println!("Formatted successfully."),
            Err(e) => eprintln!("Error formatting: {}", e),
        }
    } else {
        println!("Unknown command: {}", command);
        println!("Usage: rusty_utils format <file_path>");
    }
}
