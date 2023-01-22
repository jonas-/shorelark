use lib_simulation_wasm as sim;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: String);
}

#[wasm_bindgen]
pub fn greet(number_string: String) {
    alert(format!("From Jonas: {}!", number_string));
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let simulation = sim::Simulation::new();
    let world = simulation.world();
    log::info!("{:?}", world);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let viewport_scale = web_sys::window().unwrap().device_pixel_ratio() as f64;
    let viewport_width = canvas.width() as f64;
    let viewport_height = canvas.height() as f64;

    canvas.set_width((viewport_width * viewport_scale) as u32);
    canvas.set_height((viewport_height * viewport_scale) as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.scale(viewport_scale, viewport_scale);

    context.set_fill_style(&JsValue::from("rgb(0, 0, 0)"));

    for animal in simulation.world().animals {
        context.fill_rect(
            animal.x * viewport_width,
            animal.y * viewport_height,
            15.0,
            15.0,
        );
    }

    context.stroke();
}
