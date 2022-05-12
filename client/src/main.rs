use discover_hollywood_core::usecase::dtos::{MovieInfo, SearchQuery, SearchResponse};
use load_dotenv::try_load_dotenv;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

try_load_dotenv!();

#[function_component(SearchPage)]
fn search_page() -> Html {
    let input_ref = use_node_ref();
    let result: UseStateHandle<Vec<MovieInfo>> = use_state(|| Vec::new());

    let onchange = {
        let input_ref = input_ref.clone();
        let result = result.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>();
            if let Some(elem) = input {
                let query = SearchQuery { text: elem.value() };
                let result = result.clone();
                spawn_local(async move {
                    let client = reqwest::Client::new();
                    let resp = client
                        .get("http://localhost:8080/movies/search")
                        .query(&query)
                        .send()
                        .await
                        .unwrap()
                        .json::<SearchResponse>()
                        .await
                        .unwrap();
                    result.set(resp.movies);
                });
            }
        })
    };
    html! {
        <>
            <div>
                <h1>{ "Discover Hollywood" }</h1>
                {format!("{:?}", std::env!("HOGE"))}
            </div>
            <div>
                {"Insert a keyword and press [Enter]:"}
                <br/>
                <input type="search" ref={input_ref} {onchange} />
            </div>
            {
                result.iter().map(|movie| {
                    html! {
                        <a href={format!("http://localhost:8080/movies/{}", movie.movie.id)} style="color: inherit; text-decoration: none;">
                            <div style="border-style: solid; border-color: silver; width: 400px; white-space: pre-wrap; display:inline-block;">
                                {format!("{:#?}", movie)}
                            </div>
                        </a>
                    }
                }).collect::<Html>()
            }
        </>
    }
}

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<SearchPage>();
}
