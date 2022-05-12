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
        <a href={format!("http://localhost:8080/movies/{}", id)} style="color: inherit; text-decoration: none; border-style: solid; border-color: silver; height: 200px; width: 300px; display:inline-block;">
        <h3>{title}</h3>
            <div>
                {"Rating: "}<b>{format!("{:.2} ({} votes)", rating, rated_user_num)}</b>
            </div>
            <div>
                {"Genres: "}<b>{genres.replace("|", ", ")}</b>
            </div>
        </a>
    }
}
