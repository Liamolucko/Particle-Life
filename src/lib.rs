mod particle;
mod universe;

use std::rc::Rc;

use futures::lock::Mutex;
use universe::Settings;
use universe::Universe;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::KeyboardEvent;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    let canvas: HtmlCanvasElement = window
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into()?;

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    let universe = Rc::new(Mutex::new(Universe::new(
        canvas.width() as f64,
        canvas.height() as f64,
    )));

    {
        let mut universe = universe.lock().await;
        universe.wrap = true;
        universe.seed(9, 400, &Settings::BALANCED).await;
    }

    {
        let universe = universe.clone();

        let callback = move |ev: KeyboardEvent| {
            let universe = universe.clone();
            spawn_local(async move {
                let mut universe = universe.lock().await;
                match ev.key().as_str() {
                    "w" => universe.wrap = !universe.wrap,
                    "b" => universe.seed(9, 400, &Settings::BALANCED).await,
                    "c" => universe.seed(6, 400, &Settings::CHAOS).await,
                    "d" => universe.seed(12, 400, &Settings::DIVERSITY).await,
                    "f" => universe.seed(6, 300, &Settings::FRICTIONLESS).await,
                    "g" => universe.seed(6, 400, &Settings::GLIDERS).await,
                    "h" => universe.seed(4, 400, &Settings::HOMOGENEITY).await,
                    "l" => universe.seed(6, 400, &Settings::LARGE_CLUSTERS).await,
                    "m" => universe.seed(6, 400, &Settings::MEDIUM_CLUSTERS).await,
                    "q" => universe.seed(6, 300, &Settings::QUIESCENCE).await,
                    "s" => universe.seed(6, 600, &Settings::SMALL_CLUSTERS).await,
                    _ => {}
                }
            })
        };

        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(_)>);

        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }

    {
        let universe = universe.clone();

        let closure: Rc<Mutex<Option<Closure<dyn FnMut()>>>> = Rc::new(Mutex::new(None));
        let clone = closure.clone();

        let callback = move || {
            let ctx: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            let universe = universe.clone();
            let closure = closure.clone();
            spawn_local(async move {
                let mut universe = universe.lock().await;

                for opacity in 1..10 {
                    universe.step();
                    universe.draw(&ctx, opacity as f64 / 10.0).unwrap();
                }

                web_sys::window()
                    .unwrap()
                    .request_animation_frame(
                        closure
                            .lock()
                            .await
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .unwrap();
            });
        };

        *clone.lock().await = Some(Closure::wrap(Box::new(callback) as Box<dyn FnMut()>));

        window.request_animation_frame(
            clone
                .lock()
                .await
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )?;
    }

    Ok(())
}