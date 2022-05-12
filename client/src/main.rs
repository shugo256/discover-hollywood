mod components;

use components::search_page::SearchPage;
use load_dotenv::try_load_dotenv;

try_load_dotenv!();

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<SearchPage>();
}
