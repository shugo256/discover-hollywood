use discover_hollywood_core::{models::Movie, usecase::dtos::MovieInfo};
use load_dotenv::try_load_dotenv;
use reqwest::StatusCode;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// Loading .env file at compile time.
try_load_dotenv!();

#[derive(PartialEq, Properties)]
pub(crate) struct MoviePageProps {
    pub id: String,
}

// State of the page
#[derive(Clone, Debug, PartialEq)]
enum MoviePageState {
    Loading(String),
    NotFound,
    Found {
        movie_info: MovieInfo,
        image_uri: Option<String>,
        movie_overview: String,
    },
}

#[function_component(MoviePage)]
pub(crate) fn movie_page(MoviePageProps { id }: &MoviePageProps) -> Html {
    let page_state: UseStateHandle<MoviePageState> =
        use_state(|| MoviePageState::Loading(id.clone()));

    if let MoviePageState::Loading(movie_id) = &*page_state {
        let movie_id = movie_id.clone();
        let page_state = page_state.clone();
        spawn_local(async move {
            let url = format!("http://localhost:8080/movies/{}", movie_id);
            let resp = reqwest::get(url).await.unwrap();

            if resp.status() == StatusCode::NOT_FOUND {
                page_state.set(MoviePageState::NotFound);
            } else {
                let movie_info = resp.json::<MovieInfo>().await.ok();

                let url = format!(
                    "https://api.themoviedb.org/3/movie/{}?api_key={}",
                    movie_info.as_ref().unwrap().movie.tmdb_id,
                    std::env!("TMDB_V3_API_KEY")
                );

                let tmdb_resp = reqwest::get(url)
                    .await
                    .unwrap()
                    .json::<TmdbResponse>()
                    .await
                    .ok();

                movie_info
                    .and_then(|info| {
                        tmdb_resp.map(|tmdb| MoviePageState::Found {
                            movie_info: info,
                            image_uri: tmdb
                                .poster_path
                                .map(|path| format!("https://image.tmdb.org/t/p/w500/{}", path)),
                            movie_overview: tmdb.overview,
                        })
                    })
                    .map(|new_state| page_state.set(new_state));
            }
        });
    }

    match &*page_state {
        MoviePageState::Loading(_) => html! { <>{"Loading..."}</> },
        MoviePageState::NotFound => html! { <h1>{"NOT FOUND"}</h1> },
        MoviePageState::Found {
            movie_info:
                MovieInfo {
                    movie:
                        Movie {
                            title,
                            genres,
                            imdb_id,
                            tmdb_id,
                            ..
                        },
                    rating,
                    rated_user_num,
                },
            image_uri,
            movie_overview,
        } => html! {
            <>
            <title>{title.clone()}</title>

            <a href="http://localhost:8080/">{"< Back"}</a>

            <h1>{title.clone()}</h1>

            <div style="font-size: x-large; margin-bottom: 20px;">
                {"Rating: "}<b>{format!("{:.2} ({} votes)", rating, rated_user_num)}</b>
            </div>

            <div style="font-size: large; margin-bottom: 20px;">
                {"Genres: "}<b>{genres.replace("|", ", ")}</b>
            </div>

            <div style="display:inline-block;">
                {
                    if let Some(uri) = image_uri {
                        html! { <img src={uri.clone()} style="height: 400px; float: left;" /> }
                    } else {
                        html! {
                            <div style="height: 400px; width: 300px; float: left; border-style: solid; border-color: silver; ">
                                {"NO POSTER IMAGE"}
                            </div>
                        }
                    }
                }
                <div>
                    <div style="font-size: large; margin-bottom: 20px;">
                        {movie_overview}
                    </div>
                    <div>
                        <a href={format!("http://www.imdb.com/title/tt{}", imdb_id)}>{"[See in IMDB.]"}</a>
                        {" "}
                        <a href={format!("https://www.themoviedb.org/movie/{}", tmdb_id)}>{"[See in TMDB.]"}</a>
                    </div>
                </div>
            </div>
            </>
        },
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TmdbResponse {
    overview: String,
    poster_path: Option<String>,
}
