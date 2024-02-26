// mod utils;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{Document, Window, HtmlDocument, HtmlCanvasElement, Performance, CanvasRenderingContext2d};


mod encrypt_mod;

use crc::{crc32, Hasher32};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlDocument, Window, Performance};
use crate::encrypt_mod::encrypt;

fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("No global `window` exists"))
}

fn document() -> Result<Document, JsValue> {
    window()?.document().ok_or_else(|| JsValue::from_str("Should have a document on window"))
}

fn html_document() -> Result<HtmlDocument, JsValue> {
    document()?.dyn_into::<HtmlDocument>().map_err(|_| JsValue::from_str("Failed to convert to HtmlDocument"))
}

fn performance() -> Result<Performance, JsValue> {
    window()?.performance().ok_or_else(|| JsValue::from_str("performance should be available"))
}

fn create_canvas_with_text() -> Result<HtmlCanvasElement, JsValue> {
    let canvas = html_document()?.create_element("canvas")?;
    canvas.set_attribute("display", "none")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.begin_path();
    context.fill_style();
    context.fill_text("❤️🤪🎉👋", 50.0, 70.0)?;
    context.stroke();

    Ok(canvas)
}

fn calculate_crc32_checksum(data_url: &str) -> u32 {
    let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
    digest.write(data_url.as_bytes());
    digest.sum32()
}

fn make_fingerprint() -> Result<String, JsValue> {
    let performance = performance()?;

    let start_time = performance.now();

    let canvas = create_canvas_with_text()?;
    let data_url = canvas.to_data_url()?;
    let checksum = calculate_crc32_checksum(&data_url);

    let end_time = performance.now();

    let output = format!(
        "{{\"ms\": {:?},\"print\": \"{:X}\"}}",
        end_time - start_time,
        checksum
    );
    Ok(output)
}



#[wasm_bindgen]
pub fn get_request_session(secret: &str) -> String {
    let fingerprint = make_fingerprint().unwrap();
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

