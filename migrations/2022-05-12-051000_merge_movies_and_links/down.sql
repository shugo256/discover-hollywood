-- This file should undo anything in `up.sql`
ALTER TABLE
    movies DROP COLUMN `imdb_id`;

ALTER TABLE
    movies DROP COLUMN `tmdb_id`;

CREATE TABLE links (
    `movie_id` VARCHAR(10) PRIMARY KEY NOT NULL,
    `imdb_id` VARCHAR(10) NOT NULL,
    `tmdb_id` VARCHAR(10) NOT NULL,
    FOREIGN KEY (`movie_id`) references movies(id)
);