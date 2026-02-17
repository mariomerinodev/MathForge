use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn saludar_motor(nombre: &str) -> String {
    format!("¡Hola {}, el motor de MathForge en Rust está vivo!", nombre)
}

#[wasm_bindgen]
pub fn sumar_test(a: f64, b: f64) -> f64 {
    a + b
}