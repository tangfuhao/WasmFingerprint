use crc::{crc32, Hasher32};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlDocument, Window, Performance};


fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("No global `window` exists"))
}

fn document() -> Result<Document, JsValue> {
    window()?.document().ok_or_else(|| JsValue::from_str("Should have a document on window"))
}

fn html_document() -> Result<HtmlDocument, JsValue> {
    document()?.dyn_into::<HtmlDocument>().map_err(|_| JsValue::from_str("Failed to convert to HtmlDocument"))
}

// fn performance() -> Result<Performance, JsValue> {
//     window()?.performance().ok_or_else(|| JsValue::from_str("performance should be available"))
// }

fn create_canvas_with_text() -> Result<HtmlCanvasElement, JsValue> {
    let canvas = html_document()?.create_element("canvas")?;
    canvas.set_attribute("display", "none")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // 设置字体样式
    context.set_font("30px Arial");
    context.set_fill_style(&JsValue::from_str("red"));
    context.fill_text("❤️🤪🎉👋", 50.0, 70.0)?;

    // 在不同位置绘制多行文本
    context.set_font("20px Arial");
    context.set_fill_style(&JsValue::from_str("blue"));
    context.fill_text("🌟🍕🦄🔥", 80.0, 120.0)?;

    // 绘制一个矩形
    context.set_fill_style(&JsValue::from_str("green"));
    context.fill_rect(150.0, 50.0, 100.0, 50.0);



    // 绘制一条线
    context.begin_path();
    context.move_to(50.0, 200.0);
    context.line_to(250.0, 200.0);
    context.set_stroke_style(&JsValue::from_str("purple"));
    context.set_line_width(5.0);
    context.stroke();

    Ok(canvas)
}

fn calculate_crc32_checksum(data_url: &str) -> u32 {
    let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
    digest.write(data_url.as_bytes());
    digest.sum32()
}

pub fn make_canvas_fingerprint() -> Result<String, JsValue> {
    // let performance = performance()?;

    // let start_time = performance.now();

    let canvas = create_canvas_with_text()?;
    let data_url = canvas.to_data_url()?;
    let checksum = calculate_crc32_checksum(&data_url);
    Ok(format!("{:X}", checksum))

    // let end_time = performance.now();

    // let output = format!(
    //     "{{\"ms\": {:?},\"print\": \"{:X}\"}}",
    //     end_time - start_time,
    //     checksum
    // );
    // Ok(output)
}
