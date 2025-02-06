// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/boxed-slices.html

use crate::{GeneratorMode, Orientation, crossword};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Word {
    pub word: String,
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub orientation: String,
}

#[wasm_bindgen]
pub fn generate_crossword_js(words: Box<[String]>, mode: String) -> Result<Vec<Word>, JsValue> {
    let mode = match mode.as_str() {
        "Automatic" => GeneratorMode::Automatic,
        "InputOrder" => GeneratorMode::InputOrder,
        _ => return Err(JsValue::from_str("bad generator mode")),
    };

    let words: Vec<&str> = words.iter().map(|s| s.as_ref()).collect();
    let layout = crossword::generate_crossword(&words, mode);
    let orientation_to_string = |o: Orientation| match o {
        Orientation::Horiz => "hor".to_owned(),
        Orientation::Vert => "ver".to_owned(),
    };

    let cwords = layout
        .get_words_with_ids()
        .iter()
        .map(|(wp, id)| Word {
            word: wp.word.to_owned(),
            id: *id,
            x: wp.pos.x,
            y: wp.pos.y,
            orientation: orientation_to_string(wp.orientation),
        })
        .collect();
    Ok(cwords)
}
