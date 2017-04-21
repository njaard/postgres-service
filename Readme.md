# Introduction

If you use postgres's [service feature](https://www.postgresql.org/docs/current/static/libpq-pgservice.html)
for configuring your connections, then this is the library for you.

This is for use with the [Postgres crate](https://crates.io/crates/postgres).

# Example

This example uses the service name `mydb` and overrides
the `user` value, then makes the connection.

	extern crate postgres_service;
	extern crate postgres;

	let conn = postgres::Connection::connect(
		postgres_service::load_connect_params("mydb")
			.unwrap().user("your_user_name"),
		postgres::TlsMode::None
	).unwrap();

