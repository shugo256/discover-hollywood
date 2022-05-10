table! {
    links (movie_id) {
        movie_id -> Text,
        imdb_id -> Text,
        tmdb_id -> Text,
    }
}

table! {
    movies (id) {
        id -> Text,
        title -> Text,
        genres -> Text,
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

allow_tables_to_appear_in_same_query!(
    links,
    movies,
    ratings,
    tags,
);
