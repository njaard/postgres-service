extern crate postgres_shared;
extern crate ini;

use ini::Ini;
use ini::ini::Properties;

pub struct ServiceBuilder
{
	pg_builder : postgres_shared::params::Builder,
	maybe_host : Option<postgres_shared::params::Host>,
}

impl ServiceBuilder
{
	pub fn user(mut self, user : &str) -> ServiceBuilder
	{
		self.pg_builder.user(user, None);
		self
	}
}

impl postgres_shared::params::IntoConnectParams for ServiceBuilder
{
	fn into_connect_params(mut self)
	-> Result<postgres_shared::params::ConnectParams, Box<std::error::Error + Sync + Send>>
	{
		let host = self.maybe_host.unwrap_or(
			postgres_shared::params::Host::Unix(
				std::path::PathBuf::from("/var/run/postgresql")
			)
		);

		Ok(self.pg_builder.build(host))
	}
}


fn build_from_section(section : &Properties)
-> ServiceBuilder
{
	let mut host : Option<postgres_shared::params::Host> = None;
	let mut username : Option<String> = None;
	let mut password : Option<String> = None;

	let mut builder = postgres_shared::params::Builder::new();


	for (k,v) in section
	{
		match k.as_str()
		{
			"host" =>
				host = Some(
					if v.len()>0 && v.starts_with('/')
						{ postgres_shared::params::Host::Unix(std::path::PathBuf::from(v)) }
					else
						{ postgres_shared::params::Host::Tcp(v.clone()) }
				),
			"hostaddr" => 
				host = Some(postgres_shared::params::Host::Tcp(v.clone())),
			"port" =>
				{ builder.port(v.parse().unwrap()); },
			"dbname" =>
				{ builder.database(v); },
			"user" =>
				username = Some(v.clone()),
			"password" =>
				password = Some(v.clone()),
			_ =>
				{ builder.option(k, v); },
		}
	}

	if let Some(username) = username
		{ builder.user(&username, password.as_ref().map(|x| x.as_ref())); }

	ServiceBuilder
	{
		pg_builder : builder,
		maybe_host : host,
	}

}

pub fn load_connect_params(
	service_name : &str
) -> Option<ServiceBuilder>
{
	if let Ok(home) = std::env::var("HOME")
	{
		if let Ok(ini) = Ini::load_from_file(home + "/" + ".pg_service.conf")
		{
			if let Some(section) = ini.section(Some(service_name.clone()))
			{
				return Some(build_from_section(section));
			}
		}
	}

	let confdir = std::env::var("PGSYSCONFDIR").unwrap_or("/etc/postgresql-common".into());

	if let Ok(ini) = Ini::load_from_file(confdir + "/" + "pg_service.conf")
	{
		if let Some(section) = ini.section(Some(service_name))
		{
			return Some(build_from_section(section));
		}
	}

	None
}

