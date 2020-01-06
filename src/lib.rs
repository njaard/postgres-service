use ini::Ini;
use ini::ini::Properties;
use postgres::config::Config;

fn build_from_section(section: &Properties)
	-> Config
{
	let mut username: Option<String> = None;
	let mut password: Option<String> = None;

	let mut builder = Config::new();
	let mut options = String::new();

	for (k,v) in section.iter()
	{
		match k.as_str()
		{
			"host" =>
				{ builder.host(v); },
			"hostaddr" => 
				{ builder.host(v); },
			"port" =>
				{ builder.port(v.parse().unwrap()); },
			"dbname" =>
				{ builder.dbname(v); },
			"user" =>
				username = Some(v.clone()),
			"password" =>
				password = Some(v.clone()),
			_ =>
				options += &format!("{}={} ", k, v),
		}
	}

	if !options.is_empty()
		{ builder.options(&options); }

	if let Some(username) = username
		{ builder.user(&username); }
	if let Some(password) = password
		{ builder.password(&password); }

	builder
}

pub fn load_connect_params(
	service_name : &str
) -> Option<Config>
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

