// mod utils;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{Document, Window, HtmlDocument, HtmlCanvasElement, Performance, CanvasRenderingContext2d};


pub mod encrypt_mod;
mod utils;
pub mod session_mod;

use encrypt_mod::encrypt;
use session_mod::make_canvas_fingerprint;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn get_request_session(secret: &str) -> String {
    let fingerprint = make_canvas_fingerprint().unwrap();
    let encrypted = encrypt(&fingerprint, secret);
    encrypted
}



// fn get_fingerprint_from_cookie() -> Option<String> {
//     let cookies = html_document().cookie().unwrap();
//     for cookie in cookies.split(';') {
//         let mut key_value = cookie.splitn(2, '=');
//         let key = key_value.next().unwrap().trim();
//         let value = key_value.next()?.trim();

//         if key == "fingerprint" {
//             return Some(value.to_string());
//         }
//     }
//     None
// }


// fn set_fingerprint_to_cookie(fingerprint: &str) {
//     let cookie = format!("fingerprint={};path=/;max-age=31536000", fingerprint);
//     document().dyn_into::<HtmlDocument>().unwrap().set_cookie(&cookie).unwrap();
// }

