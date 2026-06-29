use crate::ast::{Pattern, Row};

pub fn format(pattern: &Pattern) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    for row in &pattern.rows {
        rows.push(format_row(row));
    }
    rows
}

fn format_row(row: &Row) -> String {
    let mut return_str: String = String::new();
    for instruction in &row.instructions {
        let multiplier_str: String = instruction.multiplier
            .map(|m| m.to_string())
            .unwrap_or_else(|| "?".to_string());
        return_str += &multiplier_str;
        return_str += " x ";
        return_str += &instruction.en.to_string();
        return_str += "\t";
    }
    return_str
}