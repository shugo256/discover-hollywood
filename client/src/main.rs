mod components;

use components::Main;
use load_dotenv::try_load_dotenv;

try_load_dotenv!();

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<Main>();
}
