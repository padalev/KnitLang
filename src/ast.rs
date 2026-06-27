use wasm_bindgen::prelude::*;
use crate::parser::parse;

/// The top-level AST representing the entire pattern or program.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    #[wasm_bindgen(skip)]
    pub rows: Vec<Row>,
    #[wasm_bindgen(skip)]
    flattened_rows: Option<Vec<FlatRow>>,
    #[wasm_bindgen(skip)]
    index: usize,
    #[wasm_bindgen(skip)]
    number_of_stitches: i32,
}

fn delta(instructions: Vec<Instruction>) -> i32 {
    instructions.iter().map(|i| i.delta*i.multiplier.unwrap_or(1)).sum()
}

#[wasm_bindgen]
impl Pattern {
    // 2. Create an explicit getter that clones or serializes the rows
    //#[wasm_bindgen(getter)]
    //pub fn rows(&self) -> JsValue {
        // Using serde to convert the Vec<Row> to a JS array safely
    //    serde_wasm_bindgen::to_value(&self.rows).unwrap()
    //}
    #[wasm_bindgen(getter, js_name = "row")]
    pub fn index_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.index).unwrap()
    }

    #[wasm_bindgen(skip)]
    pub fn index(&self) -> usize {
        self.index
    }

    #[wasm_bindgen(getter, js_name = "length")]
    pub fn length_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.len()).unwrap()
    }

    #[wasm_bindgen(skip)]
    pub fn len(&self) -> usize {
        self.flattened_rows.as_ref().unwrap().len()
    }

    #[wasm_bindgen]
    pub fn flatten_rows(&mut self, rows: Vec<Row>) -> Vec<FlatRow> {
        let mut flattened: Vec<FlatRow> = Vec::new();
        for row in rows {
            match row.content {
                RowContent::Instructions(cont) => {
                    self.number_of_stitches += delta(cont.clone());
                    flattened.push(FlatRow { instructions: cont, number_of_stitches: Some(self.number_of_stitches) }); //todo: number of stitches
                }
                RowContent::Rows(cont) => {
                    for _ in 0..row.multiplier.unwrap() { // todo: handle multiplier wildcard '?' what could that even mean? It's kinda ambiguous
                        flattened.extend(self.flatten_rows(cont.clone()));
                    }
                }
            }
        }
        flattened
    }

    #[wasm_bindgen]
    pub fn new(content: &str) -> Pattern {
        let rows: Vec<Row> = parse(content).unwrap();
        let mut returnPattern = Pattern {
            rows: rows.clone(),
            flattened_rows: None,
            index: 0,
            number_of_stitches: 0,
        };
        returnPattern.flattened_rows = Some(returnPattern.flatten_rows(rows));
        returnPattern
    }

    #[wasm_bindgen]
    pub fn next_row(&mut self) -> Option<FlatRow> {
        if self.index == self.flattened_rows.as_ref().unwrap().len() {
            return None
        }
        self.index += 1;
        self.flattened_rows.as_ref().unwrap().get(self.index).cloned()
    }

    #[wasm_bindgen]
    pub fn previous_row(&mut self) -> Option<FlatRow> {
        if self.index == 0 {
            return None
        }
        self.index -= 1;
        self.flattened_rows.as_ref().unwrap().get(self.index).cloned() //todo
    }

    pub fn goto_row(&mut self, index: usize) -> Option<FlatRow> {
        self.index = index;
        self.flattened_rows.as_ref().unwrap().get(index).cloned()
    }

    //#[wasm_bindgen]
    //fn flatten_rows(&self) -> Pattern {
    //    Pattern { rows: flatten_rows(self.rows.clone()) }
    //}
}

/// A Row can either be a flat list of instructions or a nested loop of rows.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Row {
    #[wasm_bindgen(skip)]
    pub content: RowContent,
    #[wasm_bindgen(skip)]
    pub multiplier: Option<i32>, // e.g., "Repeat rows 1-4 three times"
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FlatRow {
    #[wasm_bindgen(skip)]
    pub instructions: Vec<Instruction>,
    #[wasm_bindgen]
    pub number_of_stitches: Option<i32>,
}

#[wasm_bindgen]
impl FlatRow {
    #[wasm_bindgen(getter)]
    pub fn instructions(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum RowContent {
    /// A collection of individual instructions within this row.
    Instructions(Vec<Instruction>),
    /// A nested loop of rows.
    Rows(Vec<Row>),
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Instruction {
    #[wasm_bindgen(skip)]
    pub key: String,
    #[wasm_bindgen(skip)]
    pub en: String,
    pub delta: i32,
    pub multiplier: Option<i32>,
}

// todo: InstructionContent for repeats...
#[wasm_bindgen]
impl Instruction {
    #[wasm_bindgen(getter)]
    pub fn key(&self) -> String {
        self.key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn en(&self) -> String {
        self.en.clone()
    }
}

impl Instruction {
    pub fn from_key(key: String, multiplier: Option<i32>) -> Instruction {
        match key.as_str() {
            "k" => Self::knit(multiplier),
            "p" => Self::purl(multiplier),
            "sl" => Self::slip(multiplier),
            "slyf" => Self::slip_front(multiplier),
            "slyb" => Self::slip_back(multiplier),
            "skp" => Self::slip_knit_pass(multiplier),
            "kfb" => Self::knit_front_back(multiplier),
            "k2tog" => Self::knit_two_together(multiplier),
            "bor" => Self::bind_off_right(multiplier),
            "bol" => Self::bind_off_left(multiplier),
            "co" => Self::cast_on(multiplier),
            "bo" => Self::bind_off(multiplier),
            _ => panic!("Unknown instruction key: {}", key)
        }
    }

    pub fn knit(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "k".to_string(),
            en: "Knit".to_string(),
            delta: 0,
            multiplier
        }
    }

    pub fn purl(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "p".to_string(),
            en: "Purl".to_string(),
            delta: 0,
            multiplier
        }
    }

    pub fn slip(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "sl".to_string(),
            en: "Slip".to_string(),
            delta: 0,
            multiplier
        }
    }

    pub fn slip_front(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "slyf".to_string(),
            en: "Slip with yarn in front".to_string(),
            delta: 0,
            multiplier
        }
    }

    pub fn slip_back(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "slyb".to_string(),
            en: "Slip with yarn in back".to_string(),
            delta: 0,
            multiplier
        }
    }

    pub fn slip_knit_pass(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "skp".to_string(),
            en: "Slip knit pass".to_string(),
            delta: -1,
            multiplier
        }
    }
    
    pub fn knit_two_together(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "k2tog".to_string(),
            en: "Knit two together".to_string(),
            delta: -1,
            multiplier
        }
    }

    pub fn cast_on(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "co".to_string(),
            en: "Cast on".to_string(),
            delta: 1,
            multiplier
        }
    }

    pub fn bind_off(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "bo".to_string(),
            en: "Bind off".to_string(),
            delta: -1,
            multiplier
        }
    }


    pub fn bind_off_left(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "bol".to_string(),
            en: "Bind off left".to_string(),
            delta: -1,
            multiplier
        }
    }

    pub fn bind_off_right(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "bor".to_string(),
            en: "Bind off right".to_string(),
            delta: -1,
            multiplier
        }
    }

    pub fn knit_front_back(multiplier: Option<i32>) -> Instruction {
        Instruction {
            key: "kfb".to_string(),
            en: "Knit front and back".to_string(),
            delta: 1,
            multiplier
        }
    }
}