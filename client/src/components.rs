use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use self::movie_page::MoviePage;
use self::search_page::SearchPage;

/// Card component to list up movies.
mod movie_card;

/// Information page for each movie. `["/:id"]`
mod movie_page;

/// Movie search page. `["/"]`
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
            Self::MoviePage { id } => html! { <MoviePage id={id.clone()} /> },
        }
    }
}

/// Entry point component for the client app.
///
/// Uses [`AppRoute`] for routiing.
#[function_component(Main)]
pub(crate) fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(AppRoute::switch)} />
        </BrowserRouter>
    }
}
