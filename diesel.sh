#!/bin/bash
sudo apt-get install postgresql-client

# psql -h localhost -p 5432 --username=postgres --password =mypassword connect postgres in cmd

cargo install diesel_cli --no-default-features --features postgres

export DATABASE_URL=postgres://postgres:mypassword@localhost

diesel setup --database-url=$DATABASE_URL

diesel migration generate create_cats

# • diesel migration revert: Runs the down.sql of the most recent migration.
# • diesel migration redo: Runs the down.sql followed by up.sql of the most recent migration.
diesel migration run

# SELECT * FROM cats
# SET SEARCH_PATH TO work;
INSERT INTO cats (name, image_path)
VALUES ('Ragdoll', '/static/image/ragdoll.png');

# docker run --name catdex-db -e POSTGRES_PASSWORD=mypassword -p 5432:5432 -d postgres:12.3-alpine