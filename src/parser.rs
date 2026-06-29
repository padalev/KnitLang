use crate::ast::{Instruction, Row, Pattern};
//use std::iter::Peekable;
//use std::str::Chars;

//use std::iter::Peekable;
//use std::str::Lines;

/*pub fn parse(input: &str) -> Result<Vec<Row>, String> {
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
}*/




// ------------------------------------------------

// ── Tokenizer ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// An instruction key like "k", "slyf", "k2tog"
    Key(String),
    /// A numeric multiplier like 6, 22
    Number(i32),
    /// `?`  – unknown multiplier
    Question,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// Newline (row separator)
    Newline,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip spaces and carriage returns, but NOT newlines
            ' ' | '\t' | '\r' => { chars.next(); }

            '\n' => {
                tokens.push(Token::Newline);
                chars.next();
            }

            '{' => { tokens.push(Token::LBrace);  chars.next(); }
            '}' => { tokens.push(Token::RBrace);  chars.next(); }
            '?' => { tokens.push(Token::Question); chars.next(); }

            // Alphabetic → instruction key (may contain embedded digits, e.g. m1r, k2tog)
            'a'..='z' | 'A'..='Z' => {
                let mut key = String::new();
                loop {
                    // Consume a run of letters
                    while chars.peek().map_or(false, |c| c.is_alphabetic()) {
                        key.push(chars.next().unwrap());
                    }
                    // Peek at any following digits
                    if chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                        // Speculatively consume the digit run into a temp buffer
                        let mut digit_buf = String::new();
                        while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                            digit_buf.push(chars.next().unwrap());
                        }
                        if chars.peek().map_or(false, |c| c.is_alphabetic()) {
                            // Digits are embedded (e.g. "1" in "m1r") — fold into key
                            key.push_str(&digit_buf);
                            // Continue the loop to consume the remaining letters
                        } else {
                            // Digits are a trailing multiplier (e.g. "5" in "m1r5")
                            // Emit the key, then the number as separate tokens
                            tokens.push(Token::Key(key));
                            tokens.push(Token::Number(digit_buf.parse().unwrap()));
                            // Skip the post-loop Key push by jumping to the outer loop
                            break;
                        }
                    } else {
                        // No digits follow — emit the key normally
                        tokens.push(Token::Key(key));
                        break;
                    }
                }
            }

            // Digit → number (only reached for numbers not consumed inside a key)
            '0'..='9' => {
                let mut num = String::new();
                while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    num.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }

            // Skip comments or unknown characters
            _ => { chars.next(); }
        }
    }

    tokens
}

// ── Parser ───────────────────────────────────────────────────────────────────
//
// Grammar (informal):
//
//   file        ::= block_body EOF
//   block_body  ::= item*
//   item        ::= row_line | repeat_block
//   row_line    ::= instruction+ NEWLINE
//   repeat_block::= LBRACE NEWLINE block_body RBRACE NUMBER NEWLINE?
//   instruction ::= KEY (NUMBER | '?' )?
//
// After parsing, repeat blocks are expanded and flattened into plain rows.

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        // Strip leading newlines
        let mut pos = 0;
        while tokens.get(pos) == Some(&Token::Newline) {
            pos += 1;
        }
        Parser { tokens, pos }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    /// Consume zero or more consecutive newlines.
    fn skip_newlines(&mut self) {
        while self.peek() == Some(&Token::Newline) {
            self.advance();
        }
    }

    /// Parse the body of a block (or the top-level file) and return flat rows.
    /// Stops when it hits `}` or runs out of tokens.
    fn parse_block_body(&mut self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        loop {
            self.skip_newlines();

            match self.peek() {
                None | Some(Token::RBrace) => break,

                Some(Token::LBrace) => {
                    // Repeat block: { ... }N
                    let expanded = self.parse_repeat_block();
                    rows.extend(expanded);
                }

                Some(Token::Key(_)) => {
                    // Regular row of instructions
                    if let Some(row) = self.parse_row() {
                        rows.push(row);
                    }
                }

                // Unexpected token – skip it
                _ => { self.advance(); }
            }
        }

        rows
    }

    /// Parse `{ <block_body> }N` and return the body repeated N times (flat).
    fn parse_repeat_block(&mut self) -> Vec<Row> {
        // Consume `{`
        self.advance();
        self.skip_newlines();

        let inner_rows = self.parse_block_body();

        // Consume `}`
        self.skip_newlines();
        if self.peek() == Some(&Token::RBrace) {
            self.advance();
        }

        // Expect the repeat count immediately after `}`
        let repeat = if let Some(Token::Number(n)) = self.peek().cloned() {
            self.advance();
            n
        } else {
            1
        };

        // Flatten: repeat the inner rows `repeat` times
        inner_rows
            .iter()
            .cloned()
            .cycle()
            .take(inner_rows.len() * repeat as usize)
            .collect()
    }

    /// Parse a single row: one or more instructions terminated by a newline or EOF.
    fn parse_row(&mut self) -> Option<Row> {
        let mut instructions = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Key(_)) => {
                    let key = if let Some(Token::Key(k)) = self.advance().cloned() {
                        k
                    } else {
                        unreachable!()
                    };

                    // Optional multiplier: a number or `?`
                    let multiplier: Option<i32> = match self.peek() {
                        Some(Token::Number(_)) => {
                            if let Some(Token::Number(n)) = self.advance().cloned() {
                                Some(n)
                            } else {
                                None
                            }
                        }
                        Some(Token::Question) => {
                            self.advance();
                            None // `?` means unknown/variable — represented as None
                        }
                        _ => Some(1), // bare key → multiplier of 1
                    };

                    if let Some(instr) = Instruction::from_key(&key, multiplier) {
                        instructions.push(instr);
                    } else {
                        // Unknown key — could log a warning here
                        eprintln!("Warning: unknown instruction key '{}'", key);
                    }
                }

                // Row ends at newline, `{`, `}`, or EOF
                _ => break,
            }
        }

        if instructions.is_empty() {
            None
        } else {
            Some(Row { instructions: instructions, stitches_in: None, stitches_out: None })
        }
    }
}

// ── Count Stitches ────────────────────────────────────────────────────────────

fn count_stitches(rows: &mut Vec<Row>) {
    let mut prev: Vec<i32> = vec![i32::MAX];
    for row in rows.iter_mut() {
        // Pre-scan: for each instruction, compute the total known consumption
        // that follows it within the same segment. This lets `?` multipliers
        // leave room for later fixed instructions in the same segment.
        let n = row.instructions.len();
        let mut known_consumed_after = vec![0i32; n];
        {
            // Walk backwards, tracking segment boundaries and accumulating.
            let mut seg_remaining: i32 = 0;
            for j in (0..n).rev() {
                let inst = &row.instructions[j];
                match inst.key.as_str() {
                    "pm" | "sm" => {
                        // Crossing a segment boundary resets the lookahead.
                        seg_remaining = 0;
                    }
                    "rm" => {}
                    _ => {
                        // Record how much is consumed *after* this instruction
                        // (not including itself).
                        known_consumed_after[j] = seg_remaining;
                        if let Some(m) = inst.multiplier {
                            seg_remaining += inst.before * m;
                        }
                        // If multiplier is None, we can't know its consumption
                        // yet, so we don't add to seg_remaining (conservative).
                    }
                }
            }
        }

        let mut produced: Vec<i32> = vec![0];
        let mut consumed: Vec<i32> = vec![0];
        let mut i: usize = 0;

        for j in 0..n {
            let instruction = &mut row.instructions[j];
            match instruction.key.as_str() {
                "pm" => {
                    let remainder = if prev[i] == i32::MAX { i32::MAX }
                                    else { prev[i] - consumed[i] };
                    prev.insert(i + 1, remainder);
                    if prev[i] != i32::MAX { prev[i] = consumed[i]; }
                    i += 1;
                    produced.push(0);
                    consumed.push(0);
                }
                "sm" => {
                    i += 1;
                    if produced.len() <= i { produced.push(0); }
                    if consumed.len() <= i { consumed.push(0); }
                }
                "rm" => {
                    if i + 1 < prev.len() {
                        let next_prev = prev.remove(i + 1);
                        if prev[i] != i32::MAX && next_prev != i32::MAX {
                            prev[i] += next_prev;
                        } else {
                            prev[i] = i32::MAX;
                        }
                    }
                }
                _ => {
                    let before = instruction.before;
                    let after  = instruction.after;
                    let m = match instruction.multiplier {
                        Some(m) => m,
                        None => {
                            if before > 0 && prev[i] != i32::MAX {
                                let available = prev[i] - consumed[i] - known_consumed_after[j];
                                let m = (available / before).max(0);
                                instruction.multiplier = Some(m);
                                m
                            } else {
                                instruction.multiplier = Some(0);
                                0
                            }
                        }
                    };
                    consumed[i] += before * m;
                    produced[i] += after  * m;
                }
            }
        }

        row.stitches_out = Some(produced.clone());
        row.stitches_in  = Some(consumed.clone());

        produced.reverse();
        prev = produced;
    }
}

// ── Fill Multiplier ───────────────────────────────────────────────────────────

fn fill_multiplier(rows: &mut Vec<Row>) {
    for row in rows {
        let stitches_out = match &row.stitches_out {
            Some(s) => s.clone(),
            None => continue,
        };

        // First pass: accumulate known produced contributions per segment.
        let mut set_produced: Vec<i32> = vec![0; stitches_out.len()];
        let mut seg: usize = 0;
        for instruction in &row.instructions {
            match instruction.key.as_str() {
                "pm" | "sm" => { seg += 1; }
                "rm" => {}
                _ => {
                    if let Some(m) = instruction.multiplier {
                        if seg < set_produced.len() {
                            set_produced[seg] += instruction.after * m;
                        }
                    }
                }
            }
        }

        // Second pass: fill remaining None multipliers (pure producers only).
        seg = 0;
        for instruction in &mut row.instructions {
            match instruction.key.as_str() {
                "pm" | "sm" => { seg += 1; }
                "rm" => {}
                _ => {
                    if instruction.multiplier.is_none() {
                        let target = stitches_out.get(seg).copied().unwrap_or(0);
                        instruction.multiplier = Some(if instruction.after > 0 {
                            (target - set_produced[seg]) / instruction.after
                        } else {
                            0
                        });
                    }
                }
            }
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn parse(input: &str) -> Result<Vec<Row>, String> {
    let tokens: Vec<Token> = tokenize(input);
    let mut parser: Parser = Parser::new(tokens);
    let mut rows: Vec<Row> = parser.parse_block_body();
    count_stitches(&mut rows);
    fill_multiplier(&mut rows);
    Ok(rows)
}

// ── Tests ─────────────────────────────────────────────────────────────────────
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_row() {
        let rows = parse("k3 p2\n").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].instructions[0].key, "k");
        assert_eq!(rows[0].instructions[0].multiplier, Some(3));
        assert_eq!(rows[0].instructions[1].key, "p");
        assert_eq!(rows[0].instructions[1].multiplier, Some(2));
    }

    #[test]
    fn test_bare_key_gets_multiplier_one() {
        let rows = parse("k\n").unwrap();
        assert_eq!(rows[0].instructions[0].multiplier, Some(1));
    }

    #[test]
    fn test_question_mark_multiplier() {
        let rows = parse("k?\n").unwrap();
        assert_eq!(rows[0].instructions[0].multiplier, None);
    }

    #[test]
    fn test_repeat_block_flattens() {
        // { k p }3  →  3 identical rows
        let rows = parse("{\nk p\n}3\n").unwrap();
        assert_eq!(rows.len(), 3);
        for row in &rows {
            assert_eq!(row.instructions.len(), 2);
        }
    }

    #[test]
    fn test_nested_repeat_block_flattens() {
        // {
        //   { k }2   →  2 rows of [k]
        //   p        →  1 row  of [p]
        // }3
        // total: (2 + 1) * 3 = 9 rows
        let rows = parse("{\n{\nk\n}2\np\n}3\n").unwrap();
        assert_eq!(rows.len(), 9);
    }

    #[test]
    fn test_sophie_scarf_row_count() {
        let input = include_str!("Sophie Scarf.knit");
        let rows = parse(input).unwrap();
        // co6                           →  1
        // { { k? slyf3 }7  … }22       →  (7+1)*22 = 176
        // { { k? slyf3 }7  … }21       →  (7+1)*21 = 168
        // { k? slyf3 }7                →  7
        // k2 skp slyf3                 →  1
        // { k? slyf3 }5                →  5
        // bol3 bor3                    →  1
        //                         total: 359
        assert_eq!(rows.len(), 359);
    }
}*/