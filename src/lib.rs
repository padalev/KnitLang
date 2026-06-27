mod ast;
mod parser;
mod formatter;

use crate::ast::Pattern;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<Pattern, String> {
    Ok(ast::Pattern::new(input))
}