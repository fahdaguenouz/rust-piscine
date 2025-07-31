pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}
impl Security {
    pub fn verify(&self, server: Result<&str, &str>) -> String {
        match self {
            Security::Unknown => match server {
                Ok(e) => e.to_string(),
                Err(_) => server.unwrap().to_string(), 
            },

            Security::Message => match server {
                Ok(n) => n.to_string(),
                Err(_) => server.expect("ERROR: program stops").to_string(),
            },

            Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),

            Security::NotFound => match server {
                Ok(v) => v.to_string(),
                Err(e) => format!("Not found: {}", e),
            },

            Security::UnexpectedUrl => match server {
                Ok(m) => panic!("{}", m),
                Err(e) => e.to_string(),
            },
        }
    }
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    security_level.verify(server)
}