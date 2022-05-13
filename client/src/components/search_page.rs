use discover_hollywood_core::usecase::dtos::{MovieInfo, SearchQuery, SearchResponse};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::movie_card::MovieCard;

#[function_component(SearchPage)]
pub(crate) fn search_page() -> Html {
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

    let click_for_details_msg = if !result.is_empty() {
        html! {
            <p style="margin-bottom: 20px;">
                {"Click a card for details!"}
            </p>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <div>
                <h1>{ "Discover Hollywood" }</h1>
            </div>
            <div>
                {"Insert a keyword and press [Enter]:"}
                <br/>
                <input type="search" ref={input_ref} {onchange} />
                {click_for_details_msg}
            </div>
            <div style="display: flex; flex-flow: row wrap; justify-content: start; align-items: stretch; text-align: left; width 90%;">
            {
                result.iter().map(|movie| { html! {
                    <MovieCard
                        id={movie.movie.id.clone()}
                        title={movie.movie.title.clone()}
                        genres={movie.movie.genres.clone()}
                        rating={movie.rating}
                        rated_user_num={movie.rated_user_num}
                    />
                }}).collect::<Html>()
            }
            </div>
        </>
    }
}
