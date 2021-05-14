use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(js_name = processImage)]
pub fn process_image(data: Vec<u8>) {
   //let result = image::load_from_memory_with_format(&data, image::ImageFormat::PNG);


}
