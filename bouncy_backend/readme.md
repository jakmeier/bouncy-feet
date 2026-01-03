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

## Local development hints

### SQL migration error: `Result::unwrap()` on an `Err` value: VersionMismatch(NUMBER)

If you hit this error, you have probably edited one of the migration .sql files under ./db_migrations.

You should never edit checked in files. Add a new migration script at the end instead.

If your new script has an error and you want to change it, then it is best practice to drop the corresponding row in `_sqlx_migrations`, undo other changes if necessary, then re-run the migration cleanly.

```sql
DELETE FROM _sqlx_migrations
WHERE version = NUMBER_IN_ERROR;
```

However, sometimes, you may want to just change an SQL script without re-running it. For example, if you change a comment.

In such a case, it can be okay to update the checksum manually.

```bash
# Get the new checksum using sqlx-cli
sqlx migrate info --source ./db_migrations/ \
-D postgres://api_user:local_password@localhost:65432/bouncyfeet
```

```sql
UPDATE _sqlx_migrations
SET checksum = '\xCHECKSUM_HERE'
WHERE version = 3;
```

Running the checksum check again should confirm that it is now fixed.

Again, only do this for local changes. Never for checked-in queries!
