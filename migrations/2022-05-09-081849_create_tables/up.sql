-- Your SQL goes here
CREATE TABLE movies (
    `id` VARCHAR(10) PRIMARY KEY NOT NULL,
    `title` VARCHAR(10) NOT NULL,
    `genres` VARCHAR(10) NOT NULL
);

CREATE TABLE ratings (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `user_id` VARCHAR(10) NOT NULL,
    `movie_id` VARCHAR(10) NOT NULL,
    `rating` DOUBLE NOT NULL,
    `timestamp` INTEGER NOT NULL,
    FOREIGN KEY (`movie_id`) references movies(id)
);

CREATE TABLE tags (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `user_id` VARCHAR(10) NOT NULL,
    `movie_id` VARCHAR(10) NOT NULL,
    `tag` VARCHAR(10) NOT NULL,
    `timestamp` INTEGER NOT NULL,
    FOREIGN KEY (`movie_id`) references movies(id)
);

CREATE TABLE links (
    `movie_id` VARCHAR(10) PRIMARY KEY NOT NULL,
    `imdb_id` VARCHAR(10) NOT NULL,
    `tmdb_id` VARCHAR(10) NOT NULL,
    FOREIGN KEY (`movie_id`) references movies(id)
);