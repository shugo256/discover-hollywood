table! {
    movies (id) {
        id -> Text,
        title -> Text,
        genres -> Text,
        imdb_id -> Text,
        tmdb_id -> Text,
    }
}

table! {
    ratings (id) {
        id -> Nullable<Integer>,
        user_id -> Text,
        movie_id -> Text,
        rating -> Double,
        timestamp -> Integer,
    }
}

table! {
    tags (id) {
        id -> Nullable<Integer>,
        user_id -> Text,
        movie_id -> Text,
        tag -> Text,
        timestamp -> Integer,
    }
}

joinable!(ratings -> movies (movie_id));
joinable!(tags -> movies (movie_id));

allow_tables_to_appear_in_same_query!(
    movies,
    ratings,
    tags,
);
