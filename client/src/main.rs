/// Set of Yes components used in the app. (Similar to React components.)
mod components;

use components::Main;

/// Main method of the WASM client app.
fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<Main>();
}
