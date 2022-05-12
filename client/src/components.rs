use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use self::search_page::SearchPage;

/// Card component to list up movies.
mod movie_card;

/// Movie search page. (= Top page "/")
mod search_page;

/// Enum that defines the path to component mapping.
#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[at("/")]
    TopPage,
    #[at("/:id")]
    MoviePage { id: String },
}

impl AppRoute {
    fn switch(route: &Self) -> Html {
        log::info!("{:?}", route);
        match route {
            Self::TopPage => html! { <SearchPage/> },
            Self::MoviePage { id } => html! { <b>{id}</b> },
        }
    }
}

/// Entry point component for the client app.
#[function_component(Main)]
pub(crate) fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(AppRoute::switch)} />
        </BrowserRouter>
    }
}
