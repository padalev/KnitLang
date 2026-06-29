use wasm_bindgen::prelude::*;
use crate::parser::{parse};

// todo: stitches on needle

/// The top-level AST representing the entire pattern or program.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    #[wasm_bindgen(skip)]
    pub rows: Vec<Row>,
}

//fn delta(instructions: Vec<Instruction>) -> i32 {
//    instructions.iter().map(|i| i.delta*i.multiplier.unwrap_or(1)).sum()
//}

#[wasm_bindgen]
impl Pattern {
    // 2. Create an explicit getter that clones or serializes the rows
    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> JsValue {
        // Using serde to convert the Vec<Row> to a JS array safely
        serde_wasm_bindgen::to_value(&self.rows).unwrap()
    }

    /*#[wasm_bindgen(getter, js_name = "length")]
    pub fn length_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.len()).unwrap()
    }*/

    /*#[wasm_bindgen(skip)]
    pub fn len(&self) -> usize {
        self.rows.as_ref().unwrap().len()
    }*/

    #[wasm_bindgen]
    pub fn new(content: &str) -> Pattern {
        let rows: Vec<Row> = parse(content).unwrap();
        Pattern {
            rows: rows.clone(),
        }
    }
}

/*#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Row {
    #[wasm_bindgen(skip)]
    pub instructions: Vec<Instruction>,
    #[wasm_bindgen(skip)]
    pub stitches: Option<Vec<i32>>,
}*/

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Row {
    #[wasm_bindgen(skip)]
    pub instructions: Vec<Instruction>,
    #[wasm_bindgen(skip)]
    pub stitches_out: Option<Vec<i32>>,   // produced per segment (was `stitches`)
    #[wasm_bindgen(skip)]
    pub stitches_in: Option<Vec<i32>>,    // consumed per segment (new)
}

#[wasm_bindgen]
impl Row {
    #[wasm_bindgen(getter)]
    pub fn instructions(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.instructions).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn stitches_out(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.stitches_out).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn stitches_in(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.stitches_in).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Instruction {
    #[wasm_bindgen(skip)]
    pub key: String,
    #[wasm_bindgen(skip)]
    pub en: String,
    pub before: i32,
    pub after: i32,
    pub multiplier: Option<i32>,
}

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
    fn registry() -> &'static [(&'static str, &'static str, i32, i32)] {
        &[
            ("k",     "Knit",                    1,  1),
            ("p",     "Purl",                    1,  1),
            ("sl",    "Slip",                    1,  1),
            ("slyf",  "Slip with yarn in front", 1,  1),
            ("slyb",  "Slip with yarn in back",  1,  1),
            ("skp",   "Slip knit pass",          2,  1),
            ("kfb",   "Knit front and back",     1,  2),
            ("m1r",   "Make one right",          0,  1),
            ("m1l",   "Make one left",           0,  1),
            ("k2tog", "Knit two together",       2,  1),
            ("bor",   "Bind off right",          1,  0),
            ("bol",   "Bind off left",           1,  0),
            ("co",    "Cast on",                 0,  1),
            ("bo",    "Bind off",                1,  0),
            ("pm",    "Place marker",            0,  0),
            ("sm",    "Slip marker",             0,  0),
            ("rm",    "Remove marker",           0,  0),
        ]
    }

    pub fn from_key(key: &str, multiplier: Option<i32>) -> Option<Instruction> {
        Self::registry()
            .iter()
            .find(|(k, _, _, _)| *k == key)
            .map(|(k, en, before, after)| Instruction {
                key: k.to_string(),
                en: en.to_string(),
                before: *before,
                after: *after,
                multiplier,
            })
    }
}