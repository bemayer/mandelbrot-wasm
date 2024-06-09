use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlButtonElement, ImageData, window};
use js_sys::Date;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let button: HtmlButtonElement = document.create_element("button")?.dyn_into()?;
    button.set_id("render");
    button.set_text_content(Some("Render"));
    body.append_child(&button)?;

    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    canvas.set_id("canvas");
    canvas.set_width(1600);
    canvas.set_height(1200);
    body.append_child(&canvas)?;

    let render_time_p = document.create_element("p")?.dyn_into::<web_sys::Element>()?;
    render_time_p.set_id("render-time");
    body.append_child(&render_time_p)?;

    let closure = Closure::wrap(Box::new(move || {
        let canvas = document.get_element_by_id("canvas").unwrap()
            .dyn_into::<HtmlCanvasElement>().unwrap();
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let start_time = Date::now();
        let image_date = render_mandelbrot(1600, 1200).unwrap();
        ctx.put_image_data(&image_date, 0.0, 0.0).unwrap();
        let end_time = Date::now();
        let render_time_p = document.get_element_by_id("render-time").unwrap();
        render_time_p.set_text_content(Some(&format!("Render Time: {:.2} ms", end_time - start_time)));
    }) as Box<dyn FnMut()>);

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn render_mandelbrot(width: u32, height: u32) -> Result<ImageData, JsValue> {
    let mut data = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 / width as f64) * 3.5 - 2.5;
            let cy = (y as f64 / height as f64) * 2.0 - 1.0;
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iteration = 0;
            while zx * zx + zy * zy < 4.0 && iteration < 255 {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iteration += 1;
            }
            let pixel_index = (y as usize * width as usize + x as usize) * 4;
            data[pixel_index] = iteration as u8;
            data[pixel_index + 1] = iteration as u8;
            data[pixel_index + 2] = iteration as u8;
            data[pixel_index + 3] = 255;
        }
    }

    let clamped_data = Clamped(data.as_slice());
    ImageData::new_with_u8_clamped_array(clamped_data, width)
}
