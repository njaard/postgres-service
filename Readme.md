# Introduction

If you use postgres's [service feature](https://www.postgresql.org/docs/current/static/libpq-pgservice.html)
for configuring your connections, then this is the library for you.

This is for use with the [Postgres crate](https://crates.io/crates/postgres).

# Example

This example uses the service name `mydb` and overrides
the `user` value, then makes the connection.

	let conn = postgres_service::load_connect_params("mydb")
		.expect("unable to find configuration")
		.user("your_user_name")
		.connect(postgres::NoTls)
		.expect("unable to connect");

