# API server of Bouncy Feet

Handles persistent storage.

## Development

The project uses sqlx for DB queries. It checks queries at compile-time against a cache of the db layout.

Getting annoying errors like this?

```txt
error: set `DATABASE_URL` to use query macros online, or run `cargo sqlx prepare` to update the query cache
```

That's expected When adding new tables or queries. They need to be added to the cache. Use the following steps.

Requires sqlx-cli locally installed:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

You also need docker. Then run:

```bash
docker run --name bf-temp-api-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=test_db -p 9432:5432 -d postgres:latest
cargo sqlx migrate run --source ./db_migrations/ -D postgres://postgres:postgres@localhost:9432/test_db
cargo sqlx prepare -D postgres://postgres:postgres@localhost:9432/test_db
docker stop bf-temp-api-db
docker rm bf-temp-api-db
```

Or, do the same with the convenient script.

```bash
bash update_query_cache.sh
```