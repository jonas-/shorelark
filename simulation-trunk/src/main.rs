use lib_simulation as sim;
use rand::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: String);
}

#[wasm_bindgen]
pub fn greet(number_string: String) {
    alert(format!("From Jonas: {}!", number_string));
}

pub fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
) {
    context.begin_path();
    context.move_to(
        x - rotation.sin() * size * 1.5,
        y + rotation.cos() * size * 1.5,
    );
    context.line_to(
        x - (rotation + 2.0 / 3.0 * PI as f64).sin() * size,
        y + (rotation + 2.0 / 3.0 * PI as f64).cos() * size,
    );
    context.line_to(
        x - (rotation + 4.0 / 3.0 * PI as f64).sin() * size,
        y + (rotation + 4.0 / 3.0 * PI as f64).cos() * size,
    );
    context.line_to(
        x - rotation.sin() * size * 1.5,
        y + rotation.cos() * size * 1.5,
    );

    context.stroke()
}

pub fn draw_square(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, size: f64) {
    context.fill_rect(x, y, size, size);
}

pub fn draw_circle(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, radius: f64) {
    context.arc(x, y, radius, 0.0, 2.0 * PI as f64);
}

pub fn redraw(
    simulation: &mut sim::Simulation,
    context: &mut CanvasRenderingContext2d,
    viewport_width: f64,
    viewport_height: f64,
    viewport_scale: f64,
) {
    context.clear_rect(0.0, 0.0, viewport_width, viewport_height);

    let mut rng = thread_rng();
    simulation.step(&mut rng);

    // draw food
    for food in &simulation.world().foods {
        context.begin_path();
        context.arc(
            food.position().x * viewport_width,
            food.position().y * viewport_height,
            5.0,
            0.0,
            2.0 * PI as f64,
        );
        context.set_fill_style(&"rgb(0, 255, 128)".into());
        context.fill();
    }

    // draw moving birds
    for animal in &simulation.world().animals {
        draw_triangle(
            &context,
            animal.position().x * viewport_width,
            animal.position().y * viewport_height,
            5.0 * viewport_scale,
            animal.rotation.angle(),
        );
        context.set_fill_style(&"rgb(255, 255, 255)".into());
        context.fill();
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let mut simulation = sim::Simulation::new();
    let world = simulation.world();
    log::info!("{:?}", world);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let viewport_scale = web_sys::window().unwrap().device_pixel_ratio() as f64;
    let viewport_width = 800.0;
    let viewport_height = 500.0;

    canvas.set_width((viewport_width * viewport_scale) as u32);
    canvas.set_height((viewport_height * viewport_scale) as u32);

    canvas
        .set_attribute(
            "style",
            &format!("width:{viewport_width}px; height:{viewport_height}px"),
        )
        .unwrap();

    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.scale(viewport_scale, viewport_scale);

    // draw animated birds
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        redraw(
            &mut simulation,
            &mut context,
            viewport_width,
            viewport_height,
            viewport_scale,
        );
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    //context.stroke();
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
