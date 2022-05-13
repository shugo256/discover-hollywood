use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub(crate) struct MovieCardProps {
    pub id: String,
    pub title: String,
    pub genres: String,
    pub rating: f64,
    pub rated_user_num: i32,
}

#[function_component(MovieCard)]
pub(crate) fn movie_card(
    MovieCardProps {
        id,
        title,
        genres,
        rating,
        rated_user_num,
    }: &MovieCardProps,
) -> Html {
    html! {
        <a href={format!("http://localhost:8080/{}", id)} style="color: inherit; text-decoration: none; border-style: solid; border-color: silver; width: 200px; margin: 1px; padding: .8em;">
            <div style="width: 100%; height: 100%;">
                <h3>{title}</h3>
                <p>
                    {"Rating: "}<b>{format!("{:.2} ({} votes)", rating, rated_user_num)}</b>
                </p>
                <p>
                    {"Genres: "}<b>{genres.replace("|", ", ")}</b>
                </p>
            </div>
        </a>
    }
}
