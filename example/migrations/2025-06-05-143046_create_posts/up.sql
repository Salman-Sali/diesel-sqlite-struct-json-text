-- Your SQL goes here
CREATE TABLE `posts` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `title` VARCHAR NOT NULL,
  `body` TEXT NOT NULL,
  `published` BOOLEAN NOT NULL DEFAULT FALSE,
  `my_json_struct` TEXT NOT NULL
);