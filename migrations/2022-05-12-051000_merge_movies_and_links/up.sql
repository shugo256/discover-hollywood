-- Your SQL goes here
ALTER TABLE
    movies
ADD
    COLUMN `imdb_id` VARCHAR(10) NOT NULL;

ALTER TABLE
    movies
ADD
    COLUMN `tmdb_id` VARCHAR(10) NOT NULL;

DROP TABLE links;