#!/bin/bash

set -eo pipefail

cleanup() {
  docker stop bf-temp-api-db || true
  docker rm bf-temp-api-db || true
}
trap cleanup EXIT

docker run --name bf-temp-api-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=test_db -p 9432:5432 -d postgres:latest
cargo sqlx migrate run --source ./db_migrations/ -D postgres://postgres:postgres@localhost:9432/test_db
cargo sqlx prepare -D postgres://postgres:postgres@localhost:9432/test_db
