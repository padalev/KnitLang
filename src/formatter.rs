/*use crate::ast::{Pattern, Row, RowContent, flatten_rows};

pub fn format(pattern: &Pattern) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    let flattened = flatten_rows(pattern.rows.clone());
    for row in &flattened {
        rows.push(format_row(row));
    }
    rows
}

fn format_row(row: &Row) -> String {
    match &row.content {
        RowContent::Instructions(instructions) => {
            let mut instructions_str = String::new();
            for inst in instructions {
                let multi = match &inst.multiplier {
                    Some(value) => {
                        value.to_string()
                    }
                    None => {
                        "?".to_string()
                    }
                };
                instructions_str.push_str(&(inst.key.clone() + &multi + " "));
            }
            instructions_str
        }
        RowContent::Rows(_) => {
            panic!("Nested loops should be flattened")
        }
    }
}*/