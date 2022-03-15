# Introduction

If you use postgres's [service feature](https://www.postgresql.org/docs/current/static/libpq-pgservice.html)
for configuring your connections, then this is the library for you.

This is for use with the [Postgres crate](https://crates.io/crates/postgres).

# Features
* supports [tokio-postgres](https://crates.io/crates/tokio-postgres) (New in 0.19.2)
* ... and regular [postgres](https://crates.io/crates/postgres)
* search in `~/.pg_service.conf`, `$PGSYSCONFDIR/pg_service.conf`, and `/etc/postgresql-common/pg_service.conf`
* Simply generates a [postgres::Config](https://docs.rs/postgres/0.19.2/postgres/config/struct.Config.html)

# Example

This example uses the service name `mydb` and overrides
the `user` value, then makes the connection.

	let conn = postgres_service::load_connect_params("mydb")
		.expect("unable to find configuration")
		.user("your_user_name")
		.connect(postgres::NoTls)
		.expect("unable to connect");

