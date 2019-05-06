extern crate cfg_if;
extern crate wasm_bindgen;
extern crate serde_json;

mod linreg;
mod math;
mod utils;

#[macro_use]
extern crate serde_derive;

use cfg_if::cfg_if;
use linreg::LinearRegression;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinearResult {
    coeficient: f32, 
    intercept: f32, 
    accuracy: f32, 
    y_predictions: Vec<f32>
}

#[wasm_bindgen]
pub fn linear_regression(x_input: Box<[f32]>, y_input : Box<[f32]>, x_predict: Box<[f32]>) -> JsValue {
    let mut model = LinearRegression::new();
    let x_values = x_input.into_vec();
    let y_values = y_input.into_vec();
    model.fit(&x_values, &y_values);

    // the unwraps aren't great but for brevity
    let coeficient = model.coefficient.unwrap();
    let intercept = model.intercept.unwrap();
    let accuracy = model.evaluate(&x_values, &y_values);

    let x_predict_vec = x_predict.into_vec();
    let y_predictions : Vec<f32> = model.predict_list(&x_predict_vec);
    
    let result = LinearResult { coeficient, intercept, accuracy, y_predictions };

    // due to wasm not allowing us to return a tuple with an array we serialise the output
    JsValue::from_serde(&result).unwrap()
}
