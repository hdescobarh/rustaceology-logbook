mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_unumbers(left: usize, right: usize) -> Option<usize> {
    left.checked_add(right)
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(message: &str) {
    alert(&format!("Hello, {}!", message));
}

// Using str and String requieres to perform copies between the JavaScript garbage-collected heap and the Wasm linear memory
#[wasm_bindgen]
pub fn text_lenght_a(text: &str) -> usize {
    text.char_indices().count()
}

// js_sys crate allows to access directly to JavaScript memory
#[wasm_bindgen]
pub fn text_lenght_b(text: &js_sys::JsString) -> u32 {
    text.length()
}

#[wasm_bindgen]
pub struct Baz {
    field0: u32,
    field1: i32,
    pub field2: u32,
    #[wasm_bindgen(readonly)]
    pub field3: u32,
}

// const obj = new Baz(3);
// assert.equal(obj.field, 3);
// obj.field = 4;
// assert.equal(obj.field, 4);
// console.log(obj.get_contents());
#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(constructor)]
    pub fn new(field1: i32) -> Baz {
        Baz {
            field1,
            field2: 50u32,
            field3: 10u32,
            field0: 15u32,
        }
    }

    // The default name for a setter is the function's name minus the set_ prefix, and if set_ isn't a prefix of the function it's an error to not provide the name explicitly.
    #[wasm_bindgen(setter)]
    pub fn set_field1(&mut self, field: i32) {
        self.field1 = field;
    }

    // The name for a getter is by default inferred from the function name it's attached to
    #[wasm_bindgen(getter)]
    pub fn field1(&self) -> i32 {
        self.field1
    }

    pub fn get_field1(&self) -> i32 {
        self.field1
    }
}
