// todo:
// x cli to pass in the file to open
// x show knitting rows (pure content)
// - show more human readabble instructions
// x keep track of loops in current row

use std::env;
use std::fs;
use std::process;
//use std::fs::File;
//use std::io;//::{self, BufRead};
//use std::path::Path;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

fn main() {
    // Collect CLI arguments
    let args: Vec<String> = env::args().collect();

    // The first argument (index 0) is always the binary name itself
    if args.len() < 2 {
        eprintln!("Error: Please provide a file path.");
        process::exit(1);
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    match process_file(file_path) {
        Ok(vec) => navigate_vector(vec).unwrap(),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}

fn process_file(path: &str) -> io::Result<Vec<String>> {
    // Read entire file content into memory
    let content = fs::read_to_string(path)?;
    
    // Stack of vectors to manage nesting levels
    let mut stack: Vec<Vec<String>> = vec![Vec::new()];

    // Scan from \n to \n
    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        if line == "{" {
            // Push a new layer onto the stack for the upcoming sub-loop
            stack.push(Vec::new());
        } else if line.starts_with('}') {
            // Error check: hit a '}' but we aren't inside a loop
            if stack.len() < 2 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Malformed file: Found closing bracket '{}' without a matching '{{'", line)
                ));
            }

            // Pop the current loop's collected rows
            let current_loop_rows = stack.pop().unwrap();
            
            // Extract and parse the multiplier (e.g., "}3" -> 3)
            let multiplier_str = &line[1..];
            let repeat_count = multiplier_str.parse::<usize>().unwrap_or_else(|_| {
                eprintln!("Warning: Invalid loop count '{}', defaulting to 1", multiplier_str);
                1
            });

            // Expand and append these rows to the parent level below it
            if let Some(parent_level) = stack.last_mut() {
                for _ in 0..repeat_count {
                    parent_level.extend(current_loop_rows.clone());
                }
            }
        } else {
            // It's a regular row, push it to the current top level of the stack
            if let Some(current_level) = stack.last_mut() {
                current_level.push(line.to_string());
            }
        }
    }

    // Error check: if stack size > 1, a '{' was never closed
    if stack.len() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Malformed file: One or more opening brackets '{' were never closed"
        ));
    }

    // Return the completed, flattened vector
    Ok(stack.pop().unwrap())
}

fn navigate_vector(rows: Vec<String>) -> io::Result<()> {
    if rows.is_empty() {
        println!("The vector is empty.");
        return Ok(());
    }

    let mut stdout = io::stdout();
    
    // Enable raw mode so we can capture key presses immediately without waiting for Enter
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;

    let mut stitches: Vec<i16> = Vec::new();
    let mut stitchcount: i16 = 0;

    for row in &rows {
        for stitch in row.split_whitespace() {
            let first_digit_idx = stitch.find(|c: char| c.is_ascii_digit());

            let (command, multiplier_str) = match first_digit_idx {
                Some(idx) => (&stitch[..idx], &stitch[idx..]),
                None => (stitch, ""), // No numbers found, multiplier string is empty
            };

            // 2. Parse the multiplier, defaulting to 1 if none was provided
            let multiplier = multiplier_str.parse::<i16>().unwrap_or(1);

            // 3. Match the command and update your counter
            match command {
                "co" | "kfb" => { 
                    // e.g., "inc3" or "p"
                    stitchcount += multiplier;
                }
                "skp" | "bol" | "bor" => { 
                    // e.g., "dec2" or "k5"
                    stitchcount -= multiplier;
                }
                _ => {}
        }
        }
        stitches.push(stitchcount);
    }

    let mut index = 0;

    loop {
        // Clear the screen and reset cursor to top-left
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Print current item and navigation status
        print!("\rRow {} of {}\n", index + 1, rows.len());
        print!("\r{} stitches on needle\n", stitches[index]);
        print!("\r---------------------\n");
        print!("\r\n\r{}\n\r\n", rows[index].trim());
        print!("\r---------------------\n");
        print!("\rControls: [Enter / ↓] Next  |  [↑] Previous  |  [Esc / Q] Quit\n");

        stdout.flush()?;

        // Wait for a key event
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            // Support Ctrl+C to quit just in case
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                break;
            }

            match code {
                // Step Forward
                KeyCode::Enter | KeyCode::Down => {
                    if index < rows.len() - 1 {
                        index += 1;
                    }
                }
                // Step Backward
                KeyCode::Up => {
                    if index > 0 {
                        index -= 1;
                    }
                }
                // Exit conditions
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    break;
                }
                _ => {} // Ignore any other keys
            }
        }
    }

    // Clean up: Restore terminal settings back to normal
    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    
    // Clear the final state so the terminal is clean
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Exited viewer.");
    
    Ok(())
}