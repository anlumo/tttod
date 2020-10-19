#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

mod components;
mod icon_names;
use icon_names::IconName;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(fern::Output::call(|record| {
            (match record.level() {
                log::Level::Error => console::error_4,
                log::Level::Warn => console::warn_4,
                log::Level::Info => console::info_4,
                log::Level::Debug => console::debug_4,
                log::Level::Trace => console::trace_4,
            })(
                &format!(
                    "%c[{}:{}]%c {{{}}} %c{}",
                    record.file().unwrap_or("<unknown>"),
                    record
                        .line()
                        .map(|line| line.to_string())
                        .unwrap_or_else(|| "?".to_string()),
                    record.level(),
                    record.args()
                )
                .into(),
                &"color:red".into(),
                &"font-weight:bold".into(),
                &"font-weight:normal".into(),
            );
        }))
        .apply()
        .map_err(|fernerr| {
            JsValue::from_str(&format!("Failed initializing logging: {}", fernerr))
        })?;
    log::debug!("Logging initialized.");

    App::<components::Root>::new().mount_to_body();

    Ok(())
}
