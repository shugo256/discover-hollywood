mod components;

use components::Main;

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<Main>();
}
