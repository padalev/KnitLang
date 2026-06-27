use crate::ast::{Instruction, Row, RowContent};
//use std::iter::Peekable;
//use std::str::Chars;

use std::iter::Peekable;
use std::str::Lines;

pub fn parse(input: &str) -> Result<Vec<Row>, String> {
    let mut lines = input.lines().peekable();
    let rows = parse_rows(&mut lines)?;
    Ok(rows)
}

fn parse_rows(lines: &mut Peekable<Lines>) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed = clean_line(line);
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "{" {
            // Recurse to handle nested rows
            let nested_rows = parse_rows(lines)?;
            
            // The inner loop returned because it PEEKED a '}'. 
            // Now the outer frame safely consumes it and gets its multiplier!
            if let Some(close_line) = lines.next() {
                let close_trimmed = clean_line(close_line);
                if close_trimmed.starts_with('}') {
                    let multiplier_str: String = close_trimmed.chars().skip(1).collect();
                    let multiplier: i32 = multiplier_str.parse::<i32>().unwrap_or(1);
                    
                    rows.push(Row {
                        content: RowContent::Rows(nested_rows),
                        multiplier: Some(multiplier), // todo: handle "?" wildcard?
                    });
                    continue;
                }
            }
            return Err("Expected a closing line starting with '}' after a block".to_string());
        }

        // Check if the NEXT line is a closing bracket without consuming it yet
        if let Some(next_line) = lines.peek() {
            if clean_line(next_line).starts_with('}') {
                // Parse this final instruction line before exiting
                if let Some(row) = parse_instruction_line(trimmed)? {
                    rows.push(row);
                }
                return Ok(rows); // Break out! The outer frame will consume the '}'
            }
        }

        // Just a normal instruction line
        if let Some(row) = parse_instruction_line(trimmed)? {
            rows.push(row);
        }
    }

    Ok(rows)
}

/// Parses an explicit row containing sequence tokens (e.g., "k2 kfb k? slyf3")
fn parse_instruction_line(line: &str) -> Result<Option<Row>, String> {
    let mut instructions = Vec::new();

    for token in line.split_whitespace() {
        // Safe check: Ignore any brackets embedded halfway inside a text line 
        if token == "{" || token.starts_with('}') {
            continue;
        }

        if let Some(inst) = parse_token(token)? {
            instructions.push(inst);
        }
    }

    if instructions.is_empty() {
        return Ok(None);
    }

    Ok(Some(Row {
        content: RowContent::Instructions(instructions),
        multiplier: Some(1),
    }))
}

/// Parses an isolated string token into an AST Instruction block
fn parse_token(token: &str) -> Result<Option<Instruction>, String> {
    if token.is_empty() {
        return Ok(None);
    }

    let key: String;
    let multiplier: Option<i32>;

    if token.ends_with('?') {
        // Handle wildcard '?' multiplier
        multiplier = None; // Using 0 as a sentinel value for '?' (dynamic/variable)
        key = token[..token.len() - 1].to_string();
    } else if token.chars().last().map_or(false, |c| c.is_ascii_digit()) {
        // Extract the trailing digits for the multiplier
        let split_idx = token
            .char_indices()
            .rev()
            .take_while(|(_, c)| c.is_ascii_digit())
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or(token.len());

        key = token[..split_idx].to_string();
        let multiplier_str = &token[split_idx..];
        multiplier = Some(multiplier_str.parse::<i32>().unwrap_or(1));
    } else {
        // Ends with a letter (e.g., "k2tog"), multiplier is implicitly 1
        multiplier = Some(1);
        key = token.to_string();
    }

    if key.is_empty() {
        return Ok(None);
    }

    Ok(Some(Instruction::from_key(key, multiplier)))
}

/// Strips away formatting whitespace and inline metadata comments like ``
fn clean_line(line: &str) -> &str {
    let cleaned = line.trim();
    //if cleaned.starts_with('[') {
    //    if let Some(end_idx) = cleaned.find(']') {
    //        cleaned = cleaned[end_idx + 1..].trim();
    //    }
    //}
    cleaned
}